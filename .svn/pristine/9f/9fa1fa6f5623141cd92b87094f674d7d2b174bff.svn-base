<?php

namespace App\Handlers\Wechat;

use App\Handlers\Base;
use GuzzleHttp\Exception\GuzzleException;
use Hyperf\Di\Annotation\Inject;
use Psr\SimpleCache\CacheInterface;
use Psr\SimpleCache\InvalidArgumentException;

class Weixin extends Base
{
    /**
     * @Inject()
     * @var CacheInterface
     */
    protected $cache;

    protected $config;
    public function __construct($config)
    {
        $this->config = $config;
    }

    /**
     * @return array|mixed|string
     * @throws InvalidArgumentException
     */
    public function getAccessToken()
    {
        $appid = $this->config['app_id']??'';
        $key = 'wx_access_token_'.md5(__FUNCTION__.$appid);
        $cacheRow = $this->cache->get($key);
        if (empty($cacheRow)) {
            $body = [
                'appid'      => $appid,
                'secret'     => $this->config['secret']??'',
                'grant_type' => 'client_credential',
            ];
            $url = "https://api.weixin.qq.com/cgi-bin/token";
            $result = $this->curlHttp($url, 'GET', '获取微信access_token', $body);
            if (empty($result) || $result['status'] !== 'success') {
                return $result;
            }
            $cacheRow = $result['data'] ?? [];
            if ( !empty($cacheRow['errcode'])) {
                return $this->baseFailed($cacheRow['errmsg'] ?? '微信access_token获取失败', $cacheRow['errcode']);
            }
            $this->cache->set($key, $cacheRow, 5400);

        }
        return $this->baseSucceed('获取成功', $cacheRow);
    }

    /**
     * @return mixed|string
     * @throws InvalidArgumentException
     */
    public function getJsApiTicket()
    {
        $appid = $this->config['app_id']??'';
        $tokenRes = $this->getAccessToken();
        if(empty($tokenRes) || $tokenRes['status'] !== 'success'){
            return $tokenRes;
        }
        $accessToken=$tokenRes['data']['access_token']??'';
        $key = 'wx_get_ticket_'.md5(__FUNCTION__.$appid);
        $cacheRow = $this->cache->get($key);
        if (empty($cacheRow)) {
            $body = [
                'access_token' => $accessToken,
                'type'         => 'jsapi',
            ];
            $url = "https://api.weixin.qq.com/cgi-bin/ticket/getticket";
            $result = $this->curlHttp($url, 'GET', '获取微信api_ticket', $body);
            if (empty($result) || $result['status'] !== 'success') {
                return $result;
            }
            $cacheRow = $result['data'] ?? [];
            if ( !empty($cacheRow['errcode'])) {
                return $this->baseFailed($cacheRow['errmsg'] ?? '微信api_ticket获取失败', $cacheRow['errcode']);
            }
            $this->cache->set($key, $cacheRow, 5400);
        }
        return $this->baseSucceed('获取成功', $cacheRow);
    }
}