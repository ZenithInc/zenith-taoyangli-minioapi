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
use Hyperf\HttpServer\Router\Router;
use App\Middleware\MinioAuth;

Router::post('/wxmini/accesstoken','App\Controller\WxMiniController@getAccessToken');
//Router::post('/upload','App\Controller\MinioController@upload');
Router::post('/upload','App\Controller\MinioController@upload');

Router::post('/wx/accesstoken','App\Controller\WxController@getAccessToken');
Router::post('/wx/jsTicket','App\Controller\WxController@getJsApiTicket');
