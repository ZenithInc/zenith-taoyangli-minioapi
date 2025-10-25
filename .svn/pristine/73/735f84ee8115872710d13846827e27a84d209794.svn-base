<?php

namespace App\Core\Common\Facade;

use Hyperf\Utils\ApplicationContext;
use Hyperf\Logger\LoggerFactory;


class Log
{
    /**
     * get
     * 获取区分channel的日志实例
     * @param string $name
     * @return mixed
     */
    public static function get(string $name = 'hyperf')
    {
        return ApplicationContext::getContainer()->get(LoggerFactory::class)->get($name);
    }

}