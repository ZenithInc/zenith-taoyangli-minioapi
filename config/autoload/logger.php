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

$logDir = rtrim(env('LOG_DIR', BASE_PATH . '/runtime'), '/') . '/logs';
$driver = env('LOG_DRIVER', 'file');
$handlers = [];
$formatter = [
    'class' => Monolog\Formatter\LineFormatter::class,
    'constructor' => [
        'format' => "%datetime%||%channel%||%level_name%||%message%||%context%||%extra%\n",
        'dateFormat' => 'Y-m-d H:i:s',
        'allowInlineLineBreaks' => true,
    ],
];

if ($driver == 'file') {
    $handlers = [
        [
            'class' => App\Core\Common\Handler\LogFileHandler::class,
            'constructor' => [
                'stream' => $logDir.'/info.log',
                'level' => Monolog\Logger::INFO,
            ],
            'formatter' => $formatter
        ],

        [
            'class' => App\Core\Common\Handler\LogFileHandler::class,
            'constructor' => [
                'stream' => $logDir.'/debug.log',
                'level' => Monolog\Logger::DEBUG,
            ],
            'formatter' => $formatter
        ],

        [
            'class' => App\Core\Common\Handler\LogFileHandler::class,
            'constructor' => [
                'stream' => $logDir.'/error.log',
                'level' => Monolog\Logger::ERROR,
            ],
            'formatter' => $formatter
        ],
    ];
}

if ($driver == 'aliyun') {
    $handlers = [
        [
            'class' => App\Core\Common\Handler\LogAliyunHandler::class,
            'formatter' => $formatter
        ],
    ];
}


return [
    'default' => [
        'handlers' => $handlers
    ],
];
