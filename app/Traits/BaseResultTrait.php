<?php

namespace App\Traits;

trait BaseResultTrait
{

    public function result($status, $respond_data, $message)
    {
        return ['status' => $status, 'data' => $respond_data, 'message' => $message];
    }

    public function baseSucceed($message = '操作成功', $respond_data = [], $status = 'success')
    {
        return $this->result($status, $respond_data, $message);
    }

    public function baseFailed($message = '操作失败', $respond_data = [], $status = 'fail')
    {
        return $this->result($status, $respond_data, $message);
    }

}