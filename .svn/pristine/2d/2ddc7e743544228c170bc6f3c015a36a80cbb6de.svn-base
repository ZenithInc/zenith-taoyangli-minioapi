<?php

namespace App\Core\Common\Container;


use Hyperf\Di\Annotation\Inject;
use Hyperf\HttpServer\Contract\RequestInterface;
use Hyperf\HttpServer\Contract\ResponseInterface;
use Hyperf\Context\Context;
use Hyperf\Utils\Coroutine;
use App\Constants\StatusCode;
use App\Core\Common\Facade\Log;

class Response
{

    /**
     * @Inject()
     * @var RequestInterface
     */
    protected $request;

    /**
     * @Inject()
     * @var ResponseInterface
     */
    protected $response;

    protected $statusCode = 0;


    public function getStatusCode()
    {
        return $this->statusCode;
    }

    public function setStatusCode($statusCode)
    {
        $this->statusCode = $statusCode;
        return $this;
    }

    public function response($data)
    {
        return $this->response->json($data);;
    }

    public function message($message)
    {
        return $this->status([],$message);
    }

    public function success($data = [], $message = null)
    {
        $response = $this->setStatusCode(StatusCode::REQUEST_SUCCESS)->status(compact('data'),$message);
//        $executionTime = microtime(true) - Context::get('request_start_time');
//        $rbs = strlen($response->getBody()->getContents());
//        $logger = Log::get(requestEntry(Coroutine::getBackTrace()));
//        $logger->info($data['msg'],getLogArguments($executionTime,$rbs));
        return $response;
    }

    public function failed($message, $code = StatusCode::BAD_REQUEST)
    {
        return $this->setStatusCode($code)->message($message);
    }

    public function status($data = [], $message = null)
    {
        $code = $this->getStatusCode();
        $message = $message ?? statusCode::getMessage($code);
        $statusMsg = [
            'requestId' => $this->request->getHeaderLine('requestId'),
            'return_code' => $code,
            'msg' => $message,
        ];
        $responseData = array_merge($statusMsg, $data);
        return $this->response($responseData);
    }

}
