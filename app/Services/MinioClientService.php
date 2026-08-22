<?php

namespace App\Services;

use Aws\Mobile\MobileClient;
use Aws\S3\S3Client;

class MinioClientService extends BaseService
{
    protected $options = [
        'endpoint' => null,
        'version' => 'latest',
        'region' => null,
        'use_path_style_endpoint' => true,
        'credentials' => [
            'key' => null,
            'secret' => null,
        ],
        'bucket' => null,
        'prefix' => null,
        'acl' => null,
    ];

    protected static $instance;

    protected $handler;

    /**
     * @param array $options S3 client options
     */
    public function __construct($options = [])
    {
        if (empty($options)) {
            $config = config('minio');
            $options = [
                'endpoint' => $config['endpoint'] ?? null,
                'version' => 'latest',
                'region' => $config['region'] ?? null,
                'use_path_style_endpoint' => true,
                'credentials' => [
                    'key' => $config['key'] ?? null,
                    'secret' => $config['secret'] ?? null,
                ],
                'bucket' => $config['bucket'] ?? null,
                'prefix' => $config['prefix'] ?? null,
                'acl' => $config['acl'] ?? null,
            ];
        }
        $this->options = array_merge($this->options, $options);
        $this->handler = new S3Client([
            'version' => $this->options['version'],
            'region' => $this->options['region'],
            'endpoint' => $this->options['endpoint'],
            'use_path_style_endpoint' => true,
            'credentials' => [
                'key' => $this->options['credentials']['key'],
                'secret' => $this->options['credentials']['secret'],
            ],
        ]);
    }

    public static function getInstance()
    {
        if (empty(self::$instance)) {
            self::$instance = new MobileClient([]);
        }
        return self::$instance;
    }

    public function __call($name, $arguments)
    {
        return call_user_func_array([$this->handler, $name], $arguments);
    }
}
