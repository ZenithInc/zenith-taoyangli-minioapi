<?php
declare(strict_types=1);
namespace App\Middleware;

use Psr\Container\ContainerInterface;
use Hyperf\HttpServer\Contract\RequestInterface;
use Hyperf\HttpServer\Contract\ResponseInterface as HttpResponse;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\ServerRequestInterface;
use Psr\Http\Server\MiddlewareInterface;
use Psr\Http\Server\RequestHandlerInterface;

class RequestAuthMiddleware implements MiddlewareInterface
{
    protected $accountRepository;
    /**
     * @var ContainerInterface
     */
    protected $container;

    /**
     * @var RequestInterface
     */
    protected $request;
    /**
     * @var HttpResponse
     */
    protected $response;
    public function __construct(ContainerInterface $container,RequestInterface $request,HttpResponse $response)
    {
        $this->container = $container;
        $this->request = $request;
        $this->response = $response;
    }

    public function process(ServerRequestInterface $request, RequestHandlerInterface $handler): ResponseInterface
    {
        $appid = $this->request->header('appid') ?? '';
        if(empty($appid)){
            return $this->response->json(
                [
                    'return_code' => 100030,
                    'msg' => 'appid不能为空',
                    'data' => [],
                ]
            );
        }
        return $handler->handle($request);
    }
}
