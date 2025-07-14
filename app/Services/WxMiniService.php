<?php

namespace App\Services;
use App\Handlers\Wechat\MiniApp;
use App\Services\BaseService;
use Hyperf\DbConnection\Db;
use Hyperf\Di\Annotation\Inject;

class WxMiniService extends BaseService
{
    /**
     * 获取微信小程序accesstoken
     * @param $param
     * @return array
     * @throws \Psr\SimpleCache\InvalidArgumentException
     */
    public function getAccessToken($param)
    {
        $mini = new MiniApp($param);
        return $mini->getAccessToken();
    }
}
