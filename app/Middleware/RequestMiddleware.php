<?php
declare(strict_types=1);
namespace App\Middleware;

use Psr\Container\ContainerInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Server\MiddlewareInterface;
use Psr\Http\Message\ServerRequestInterface;
use Psr\Http\Server\RequestHandlerInterface;
use Hyperf\Context\Context;

class RequestMiddleware implements MiddlewareInterface
{
    /**
     * @var ContainerInterface
     */
    protected $container;

    /**
     * @var ServerRequestInterface
     */
    protected $request;

    public function __construct(ContainerInterface $container,ServerRequestInterface $request)
    {
        $this->container = $container;
        $this->request = $request;
    }

    public function process(ServerRequestInterface $request, RequestHandlerInterface $handler): ResponseInterface
    {
        // 为每一个请求增加一个qid
        $request = Context::override(ServerRequestInterface::class, function (ServerRequestInterface $request) {
            $request = $request->withAddedHeader('requestId', $this->getRequestId());
            //var_dump($request->getQueryParams());
            return $request;
        });
        Context::set('request_start_time',microtime(true));
        return $handler->handle($request);
    }

    /**
     * getRequestId
     * 唯一请求id
     * @return string
     */
    protected function getRequestId()
    {
        $tmp = $this->request->getServerParams();
        $name = strtoupper(substr(md5(gethostname()), 12, 8));
        $remote = strtoupper(substr(md5($tmp['remote_addr']),12,8));
        $ip = strtoupper(substr(md5(getServerLocalIp()), 14, 4));
        return uniqid(). '-' . $remote. '-'.$ip.'-'. $name;
    }
}