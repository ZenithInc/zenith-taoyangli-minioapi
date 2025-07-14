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
namespace App\Exception\Handler;

use Hyperf\Di\Annotation\Inject;
use App\Constants\StatusCode;
use App\Core\Common\Facade\Log;
use App\Exception\BusinessException;
use Hyperf\ExceptionHandler\ExceptionHandler;
use Hyperf\HttpMessage\Stream\SwooleStream;
use Psr\Http\Message\ResponseInterface;
use App\Core\Common\Container\Response;
use Throwable;

class AppExceptionHandler extends ExceptionHandler
{

    /**
     * @Inject
     * @var Response
     */
    protected $response;


    public function handle(Throwable $throwable, ResponseInterface $response)
    {
        $throwableMsg = sprintf('%s[%s] in %s', $throwable->getMessage(), $throwable->getLine(), $throwable->getFile()).PHP_EOL.$throwable->getTraceAsString();
        $logName = requestEntry($throwable->getTrace());
        $logger = Log::get($logName);
        if ($throwable instanceof BusinessException) {
            // 阻止异常冒泡
            $this->stopPropagation();
            // 业务逻辑错误日志处理
            $logger->warning($throwableMsg,getLogArguments());
            return $this->response->failed($throwable->getCode(),$throwable->getMessage());
        }
        $logger->error($throwableMsg,getLogArguments());
        //$msg = !empty($throwable->getMessage())?$throwable->getMessage():StatusCode::getMessage(StatusCode::SERVER_ERROR);
        $msg = '服务错误';
        $data = responseDataFormat(StatusCode::SERVER_ERROR,$msg);
        $dataStream = new SwooleStream(json_encode($data, JSON_UNESCAPED_UNICODE));
        return $response->withAddedHeader('content-type', 'text/html; charset=utf-8')
            ->withStatus(StatusCode::SERVER_ERROR)
            ->withBody($dataStream);
    }

    public function isValid(Throwable $throwable): bool
    {
        return true;
    }
}
