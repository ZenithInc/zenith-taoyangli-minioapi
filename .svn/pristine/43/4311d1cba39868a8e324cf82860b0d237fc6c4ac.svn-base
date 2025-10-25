<?php

namespace App\Repositories;

abstract class  BaseRepository
{

    /**
     * @var \Hyperf\Database\Model\Model
     */
    protected $model;

    public function __construct()
    {
        $modelClass  = str_replace(['\Repositories', 'Repository'], ['\Model', ''], get_class($this));
        $this->model = make($modelClass);
    }

}