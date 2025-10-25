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

namespace App\Services;

use App\Handlers\Wechat\Weixin;

class WxService extends BaseService
{
    /**
     * 获取微信accesstoken.
     * @return array
     * @throws \Psr\SimpleCache\InvalidArgumentException
     */
    public function getAccessToken($param)
    {
        $wx = new Weixin($param);
        return $wx->getAccessToken();
    }

    /**
     * 获取微信api_ticket.
     * @return array
     * @throws \Psr\SimpleCache\InvalidArgumentException
     */
    public function getJsApiTicket($param)
    {
        $wx = new Weixin($param);
        return $wx->getJsApiTicket();
    }
}
