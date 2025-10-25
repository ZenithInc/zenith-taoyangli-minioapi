<?php

namespace App\Request;

use Hyperf\Di\Annotation\Inject;
use App\Core\Common\Container\Response;
use Hyperf\HttpServer\Contract\RequestInterface;
use Hyperf\Validation\Contract\ValidatorFactoryInterface;
use App\Traits\BaseResultTrait;
use Psr\Container\ContainerInterface;

class BaseRequest
{
    use BaseResultTrait;

    /**
     * @Inject()
     * @var ValidatorFactoryInterface
     */
    protected $validationFactory;

    protected $messages   = [
        'required'    => ':attribute不能为空',
        'between'     => ':attribute只能在:min-:max个字符范围',
        'unique'      => ':attribute已经被占用',
        'in'          => ':attribute只能是:values',
        'integer'     => ':attribute必须为整数',
        'numeric'     => ':attribute必须为数值',
        'min'         => ':attribute不能小于:min',
        'max'         => ':attribute不能大于:max',
        'array'       => ':attribute必须是合法的数组形式',
        'required_if' => ':attribute字段是必须的当:other是:value',
        'not_in'      => '选定的:attribute是无效的',
        'regex'       => ':attribute格式不正确',
        'alpha_letter_num'  => ':attribute只能是字母、数字、破折号(-)以及下划线(_)',
        'alpha_num'   => ':attribute只能是字母、数字',
        'date'        => ':attribute必须是日期字符串',
        'sometimes'   => '如果存在',
        'required_without'=>':attribute字段不能为空,当:values不存在时',
        'required_with'=>':attribute字段不能为空,当:vaules存在时'
    ];


//    public  function  __construct(ValidatorFactoryInterface $validationFactory)
//    {
//        $this->validationFactory = $validationFactory;
//
//    }

    /**
     * 验证
     */
    public function validate(array $data, array $rules, array $messages = [], array $customAttributes = [])
    {
        $validator = $this->validationFactory->make($data, $rules, $messages, $customAttributes);
        if ($validator->fails()){
            return $validator->errors()->first();
        }
        return  true;
    }





}
