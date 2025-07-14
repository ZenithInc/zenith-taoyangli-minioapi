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
namespace App\Controller;

use Hyperf\HttpServer\Contract\RequestInterface;
use Psr\Container\ContainerInterface;
use App\Core\Common\Container\Response;

abstract class AbstractController
{
    protected $container;

    protected $request;

    protected $response;


    public  function  __construct(ContainerInterface $container, RequestInterface $request, Response $response)
    {
        $this->container = $container;
        $this->request = $request;
        $this->response =$response;
    }


}
