<?php

declare(strict_types=1);
/**
 * This file is part of Hyperf.
 *
 * @link     https://www.hyperf.io
 * @document https://hyperf.wiki
 * @contact  group@hyperf.io
 * @license  https://github.com/hyperf/hyperf/blob/master/LICENSE
 */
return [
    'default' => [
        'host' => env('REDIS_HOST', 'localhost'),
        'auth' => env('REDIS_AUTH', null),
        'port' => (int) env('REDIS_PORT', 6379),
        'db' => (int) env('REDIS_DB', 0),
//        'cluster' => [
//            'enable' => (bool) env('REDIS_CLUSTER_ENABLE', false),
//            'name' => null,
//            'seeds' => explode(';', env('REDIS_CLUSTER_NODE', '')),
//        ],
        'pool' => [
            'min_connections' => 1,
            'max_connections' => 10,
            'connect_timeout' => 10.0,
            'wait_timeout' => 3.0,
            'heartbeat' => -1,
            'max_idle_time' => (float) env('REDIS_MAX_IDLE_TIME', 60),
        ],
    ],
    // 数据缓存
    'cache' => [
        'host' => env('CACHE_REDIS_HOST', 'localhost'),
        'auth' => env('CACHE_REDIS_AUTH', null),
        'port' => (int) env('CACHE_REDIS_PORT', 6379),
        'db' => (int) env('CACHE_REDIS_DB', 0),
//        'cluster' => [
//            'enable' => (bool) env('REDIS_CLUSTER_ENABLE', false),
//            'name' => null,
//            'seeds' => explode(';', env('REDIS_CLUSTER_NODE', '')),
//        ],
        'pool' => [
            'min_connections' => 1,
            'max_connections' => 20,
            'connect_timeout' => 10.0,
            'wait_timeout' => 3.0,
            'heartbeat' => -1,
            'max_idle_time' => (float) env('CACHE_REDIS_MAX_IDLE_TIME', 60),
        ],
    ],
    // 模型缓存用redis配置
    'model_cache' => [
        'host' => env('MODEL_CACHE_REDIS_HOST', 'localhost'),
        'auth' => env('MODEL_CACHE_REDIS_HOST', null),
        'port' => (int) env('MODEL_CACHE_REDIS_PORT', 6379),
        'db' => (int) env('MODEL_CACHE_REDIS_DB', 0),
//        'cluster' => [
//            'enable' => (bool) env('REDIS_CLUSTER_ENABLE', false),
//            'name' => null,
//            'seeds' => explode(';', env('REDIS_CLUSTER_NODE', '')),
//        ],
        'pool' => [
            'min_connections' => 1,
            'max_connections' => 20,
            'connect_timeout' => 10.0,
            'wait_timeout' => 3.0,
            'heartbeat' => -1,
            'max_idle_time' => (float) env('MODEL_CACHE_REDIS_MAX_IDLE_TIME', 60),
        ],
    ],
];
