<?php

namespace App\Core\Common\Handler;

use Hyperf\Utils\ApplicationContext;
use Hyperf\HttpServer\Contract\RequestInterface;
use App\Core\Common\Facade\Log;

class WriteLog
{
    public static function debug($log)
    {
        if(config('log_enable')) {
            $request = ApplicationContext::getContainer()->get(RequestInterface::class);
            $uri = $request->getRequestUri();
            $moduleName = str_replace('/','-',ltrim($uri,'/'));
            $logger = Log::get($moduleName);
            $logger->debug($log);
        }
    }

    public static function info($log)
    {
        if(config('log_enable')) {
            $request = ApplicationContext::getContainer()->get(RequestInterface::class);
            $uri = $request->getRequestUri();
            $moduleName = str_replace('/','-',ltrim($uri,'/'));
            $logger = Log::get($moduleName);
            $logger->info($log);
        }
    }

    public static function error($log)
    {
        if(config('log_enable')) {
            $request = ApplicationContext::getContainer()->get(RequestInterface::class);
            $uri = $request->getRequestUri();
            $moduleName = str_replace('/','-',ltrim($uri,'/'));
            $logger = Log::get($moduleName);
            $logger->error($log);
        }
    }

}