<?php

namespace App\Controller;

use App\Constants\StatusCode;

class BaseController extends  AbstractController
{
    public function success($data = [], $message = null)
    {
       return $this->response->success($data, $message);
    }

    public  function failed($message, $code = StatusCode::BAD_REQUEST)
    {
       return $this->response->failed($message, $code);
    }

}