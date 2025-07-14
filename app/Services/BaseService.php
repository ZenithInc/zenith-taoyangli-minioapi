<?php

namespace App\Services;

use Psr\Container\ContainerInterface;
use Hyperf\Di\Annotation\Inject;
use App\Constants\StatusCode;
use App\Traits\BaseResultTrait;

class BaseService
{
    use BaseResultTrait;
    /**
     * @Inject
     * @var ContainerInterface
     */
    protected $container;



    /**
     * 隐式注入服务类
     * @param $key
     * @return \Psr\Container\ContainerInterface|void
     */
    public function __get($key)
    {
        if ($key == 'app') {
            return $this->container;
        } elseif (substr($key, -10) == 'Repository') {
             return $this->getRepositoriesInstance($key);
        } else {
            throw new \RuntimeException("仓储{$key}不存在，书写错误！", StatusCode::SERVER_ERROR);
        }
    }



    /**
     * 获取仓储类实例
     * @param $key
     * @return mixed
     */
    public function getRepositoriesInstance($key)
    {
        $key = ucfirst($key);
        $fileName = BASE_PATH."/app/Repositories/{$key}.php";
        $className = "App\\Repositories\\{$key}";

        if (file_exists($fileName)) {
            return $this->container->get($className);
        } else {
            throw new \RuntimeException("仓储{$key}不存在，文件不存在！", StatusCode::SERVER_ERROR);
        }
    }


}
