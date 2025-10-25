<?php

namespace App\Handlers;
use App\Traits\BaseResultTrait;
use App\Repositories\ExternalLogRepository;
use Hyperf\Di\Annotation\Inject;
use Psr\SimpleCache\CacheInterface;

class Base
{
    /**
     * @Inject()
     * @var CacheInterface
     */
    protected $cache;
    /**
     * @Inject()
     * @var ExternalLogRepository
     */
    public $externalLogRepository;

    use BaseResultTrait;

    /**
     * curl
     * @param string $url
     * @param string $body
     * @param string $method
     * @param array $headers
     * @return array
     */
    function curlHttp($url, $method = 'GET', $desc = '', $body = '', $headers = [])
    {
        $requireTime = time();
        // 创建 GuzzleHttp 客户端实例
        $client = new \GuzzleHttp\Client();
        // 根据请求方法和参数创建 GuzzleHttp 请求实例
        $requestOptions = [];
        switch ($method) {
            case 'POST':
                $requestOptions['form_params'] = $body;
                break;
            case 'FILE':
                $requestOptions['multipart'] = $body;
                break;
            case 'GET':
                $requestOptions['query'] = $body;
                break;
        }
        $requestOptions['headers'] = $headers;
        // 发送请求并获取响应
        try {
            $response = $client->request($method, $url, $requestOptions);
            $content = $response->getBody()->getContents();
            $code = $response->getStatusCode();
            $error = '';
        } catch (\GuzzleHttp\Exception\ClientException $e) {
            $response = $e->getResponse();
            $content = $response->getBody()->getContents();
            $code = $response->getStatusCode();
            $error = $e->getMessage();
        }
        $this->createLog($desc,$url,is_array($body) ? json_encode($body,JSON_UNESCAPED_UNICODE) : $body,$requireTime,!empty($error) ? $error : $content);
        if (empty($code) || $code != 200 || !empty($error)) {
            return $this->baseFailed($error ?? '接口请求失败');
        } else {
            $responseData = json_decode($content, true);
            return $this->baseSucceed('接口请求成功',$responseData);
        }
    }

    /**
     * 记录日志
     * @param $method
     * @param $url
     * @param $require
     * @param $require_time
     * @param $response
     * @return void
     */
    public function createLog($method,$url,$require,$require_time,$response)
    {
        $this->externalLogRepository->createDo([
            'method' => $method,
            'url' => $url,
            'require' => is_array($require) ? json_encode($require,JSON_UNESCAPED_UNICODE) : $require,
            'require_time' => $require_time,
            'response' => is_array($response) ? json_encode($response,JSON_UNESCAPED_UNICODE) : $response,
        ]);
    }
}