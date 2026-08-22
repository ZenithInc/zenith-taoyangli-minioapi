#!/usr/bin/env python3
"""Validate that peak Pod requests stay within a node allocatable threshold."""

from __future__ import annotations

import argparse
import json
import math
import re
from pathlib import Path

import yaml

CPU = re.compile(r"^(\d+(?:\.\d+)?)([numkM]?)$")
MEMORY = re.compile(r"^(\d+(?:\.\d+)?)([EPTGMK]i?|[eptgmk])?$")
MEMORY_FACTORS = {
    "": 1, "K": 1000, "M": 1000**2, "G": 1000**3, "T": 1000**4,
    "P": 1000**5, "E": 1000**6, "Ki": 1024, "Mi": 1024**2,
    "Gi": 1024**3, "Ti": 1024**4, "Pi": 1024**5, "Ei": 1024**6,
    "k": 1000, "m": 0.001, "g": 1000**3, "t": 1000**4,
    "p": 1000**5, "e": 1000**6,
}


def cpu_millicores(value: object) -> int:
    if value in (None, ""):
        return 0
    match = CPU.fullmatch(str(value))
    if match is None:
        raise ValueError(f"unsupported CPU quantity: {value}")
    factors = {"": 1000, "m": 1, "u": 0.001, "n": 0.000001,
               "k": 1_000_000, "M": 1_000_000_000}
    return math.ceil(float(match.group(1)) * factors[match.group(2)])


def memory_bytes(value: object) -> int:
    if value in (None, ""):
        return 0
    match = MEMORY.fullmatch(str(value))
    if match is None:
        raise ValueError(f"unsupported memory quantity: {value}")
    return math.ceil(float(match.group(1)) * MEMORY_FACTORS[match.group(2) or ""])


def pod_requests(spec: dict) -> tuple[int, int]:
    regular = [
        (
            cpu_millicores(item.get("resources", {}).get("requests", {}).get("cpu")),
            memory_bytes(item.get("resources", {}).get("requests", {}).get("memory")),
        )
        for item in spec.get("containers", [])
    ]
    init = [
        (
            cpu_millicores(item.get("resources", {}).get("requests", {}).get("cpu")),
            memory_bytes(item.get("resources", {}).get("requests", {}).get("memory")),
        )
        for item in spec.get("initContainers", [])
    ]
    overhead = spec.get("overhead", {})
    return (
        max(sum(item[0] for item in regular), max((item[0] for item in init), default=0))
        + cpu_millicores(overhead.get("cpu")),
        max(sum(item[1] for item in regular), max((item[1] for item in init), default=0))
        + memory_bytes(overhead.get("memory")),
    )


def surge_count(deployment: dict, replicas: int) -> int:
    strategy = deployment.get("spec", {}).get("strategy", {})
    if strategy.get("type", "RollingUpdate") == "Recreate":
        return 0
    value = strategy.get("rollingUpdate", {}).get("maxSurge", "25%")
    if isinstance(value, str) and value.endswith("%"):
        return math.ceil(replicas * float(value[:-1]) / 100)
    return int(value)


def calculate(nodes: dict, pods: dict, documents: list[dict], release: str) -> tuple[int, int, int, int]:
    alloc_cpu = sum(cpu_millicores(node["status"]["allocatable"]["cpu"]) for node in nodes["items"])
    alloc_memory = sum(memory_bytes(node["status"]["allocatable"]["memory"]) for node in nodes["items"])
    used_cpu = used_memory = 0
    for pod in pods["items"]:
        if pod.get("status", {}).get("phase") in {"Succeeded", "Failed"}:
            continue
        if pod.get("metadata", {}).get("labels", {}).get("app.kubernetes.io/instance") == release:
            continue
        cpu, memory = pod_requests(pod.get("spec", {}))
        used_cpu += cpu
        used_memory += memory
    desired_cpu = desired_memory = 0
    for document in documents:
        if not document or document.get("kind") != "Deployment":
            continue
        replicas = int(document.get("spec", {}).get("replicas", 1))
        peak = replicas + surge_count(document, replicas)
        cpu, memory = pod_requests(document["spec"]["template"]["spec"])
        desired_cpu += cpu * peak
        desired_memory += memory * peak
    return used_cpu + desired_cpu, used_memory + desired_memory, alloc_cpu, alloc_memory


def self_test() -> int:
    assert cpu_millicores("250m") == 250
    assert cpu_millicores("2") == 2000
    assert memory_bytes("256Mi") == 256 * 1024 * 1024
    assert surge_count({"spec": {"strategy": {"type": "Recreate"}}}, 1) == 0
    assert surge_count({"spec": {"strategy": {"rollingUpdate": {"maxSurge": "25%"}}}}, 3) == 1
    print("capacity self-test passed")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--nodes", type=Path)
    parser.add_argument("--pods", type=Path)
    parser.add_argument("--rendered", type=Path)
    parser.add_argument("--release")
    parser.add_argument("--limit-percent", type=float, default=70.0)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if not all((args.nodes, args.pods, args.rendered, args.release)):
        parser.error("--nodes, --pods, --rendered and --release are required")
    nodes = json.loads(args.nodes.read_text(encoding="utf-8"))
    pods = json.loads(args.pods.read_text(encoding="utf-8"))
    documents = list(yaml.safe_load_all(args.rendered.read_text(encoding="utf-8")))
    peak_cpu, peak_memory, alloc_cpu, alloc_memory = calculate(
        nodes, pods, documents, args.release
    )
    cpu_percent = 100 * peak_cpu / alloc_cpu if alloc_cpu else 100
    memory_percent = 100 * peak_memory / alloc_memory if alloc_memory else 100
    print(
        f"capacity cpu={peak_cpu}m/{alloc_cpu}m({cpu_percent:.2f}%) "
        f"memory={peak_memory}/{alloc_memory}({memory_percent:.2f}%)"
    )
    if cpu_percent > args.limit_percent or memory_percent > args.limit_percent:
        print(f"capacity check failed: peak requests exceed {args.limit_percent:.0f}%")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
