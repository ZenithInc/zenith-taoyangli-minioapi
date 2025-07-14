<?php

namespace App\Handlers\Wechat;
use App\Handlers\Base;

class MiniApp extends Base
{
    protected $config;
    public function __construct($config)
    {
        $this->config = $config;
    }
    /**
     * 获取accesstoken
     * @return array
     * @throws \Psr\SimpleCache\InvalidArgumentException
     */
    public function getAccessToken()
    {
        $key = $this->config['app_id'].'wx_access_token';
        $cacheRow = $this->cache->get($key);
        if(empty($cacheRow)){
            $required = [
                'appid' => $this->config['app_id']??'',
                'secret' => $this->config['secret']??'',
                'grant_type' => 'client_credential',
            ];
            $res = $this->curlHttp('https://api.weixin.qq.com/cgi-bin/token','GET','微信小程序获取accesstoken',$required);
            if(empty($res) || $res['status'] !== 'success'){
                return $res;
            }
            $cacheRow = $res['data'];
            if(!empty($cacheRow['errcode'])){
                return $this->baseFailed($cacheRow['errmsg'] ?? '小程序accesstoken获取失败',$cacheRow['errcode']);
            }
            $this->cache->set($key,$cacheRow,7000);
        }
        return $this->baseSucceed('获取成功',$cacheRow);
    }
}