<?php

namespace App\Services;
use App\Services\BaseService;
use Aws\Mobile\MobileClient;
use Hyperf\DbConnection\Db;
use Hyperf\Di\Annotation\Inject;
use Aws\S3\S3Client;

class MinioClientService extends BaseService
{
    protected $options = [
        'endpoint'  =>  'https://alpha-tyfile.app.douya.wang',
        'version' => 'latest',
        'region'  => 'cn-north-1',  //China (Beijing)
        'use_path_style_endpoint' => true,
        'credentials' => [
            'key'    => 'AKIADTSFODNN7EXAMTRYL',
            'secret' => 'wKalrXUtnTEMI/P7MDENT/bPxRfiCYEXAMMDEKGA',
        ],
        'bucket' => 'tyl',
        'prefix'    => 'sample_apk/',   //自定义的键名，bucket作为桶的名字，是顶层的文件目录；剩余下级目录的表示，通过Key来实现;会存在桶名下的prefix目录下
        'acl'    => 'public-read',
    ];

    protected static $instance;

    protected $handler;

    /**
     * 构造函数
     * @param array $options 缓存参数
     * @access public
     */
    public function __construct($options = [])
    {
        if(empty($options)) {
            $options = config('minio');    //构造没穿参数，就从配置读取
            if (!empty($options)) {
                $this->options = array_merge($this->options, $options);
            }
        }

        $this->handler = new S3Client([
            'version' => $this->options['version'],
            'region'  => $this->options['region'],
            'endpoint' => $this->options['endpoint'],
            'use_path_style_endpoint' => true,
            'credentials' => [
                'key'    => $this->options['credentials']['key'],
                'secret' => $this->options['credentials']['secret'],
            ],
        ]);
    }
    /**
     * 单例模式 获取实例
     * @return MobileClient
     */
    public static  function getInstance()
    {
        if(empty(self::$instance)) {
            self::$instance = new MobileClient([]);
        }
        return self::$instance;
    }

    /**
     * call me
     * @param $name
     * @param $arguments
     * @return mixed
     */
    public function __call($name, $arguments)
    {
        return call_user_func_array(
            array($this->handler, $name),
            $arguments);
    }
}
