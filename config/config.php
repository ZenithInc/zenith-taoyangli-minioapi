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
use Hyperf\Contract\StdoutLoggerInterface;
use Psr\Log\LogLevel;

return [
    'app_name' => env('APP_NAME', 'skeleton'),
    'app_env' => env('APP_ENV', 'dev'),
    'scan_cacheable' => env('SCAN_CACHEABLE', false),
    // 是否记录日志
    'log_enable' => env('LOG_ENABLE', false),
    // 是否记录框架的日志
    'hf_log' => env('HF_LOG', false),
    StdoutLoggerInterface::class => [
        'log_level' => [
            LogLevel::ALERT,
            LogLevel::CRITICAL,
            LogLevel::DEBUG,
            LogLevel::EMERGENCY,
            LogLevel::ERROR,
            LogLevel::INFO,
            LogLevel::NOTICE,
            LogLevel::WARNING,
        ],
    ],
    'client_app_id' => env('CLIENT_APP_ID',''),
    'client_app_secret' => env('CLIENT_APP_SECRET',''),
    'client_app_key' => env('CLIENT_APP_KEY',''),

    'client_app_id_second' => env('CLIENT_APP_ID_SECOND',''),
    'client_app_secret_second' => env('CLIENT_APP_SECRET_SECOND',''),
    'client_app_key_second' => env('CLIENT_APP_KEY_SECOND',''),
];
