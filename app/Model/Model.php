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
namespace App\Model;

use Hyperf\DbConnection\Model\Model as BaseModel;
use Hyperf\ModelCache\Cacheable;
use Hyperf\ModelCache\CacheableInterface;
use Hyperf\Utils\Str;

abstract class Model extends BaseModel implements  CacheableInterface
{
    use Cacheable;

    const CREATED_AT = 'dateline';

    const UPDATED_AT = 'uptime';

    protected $dateFormat = 'U';
    /**
     * 通过主键id/ids获取信息
     * @param $id
     * @param bool $useCache 是否使用模型缓存
     * @return array
     */
    public function getOneById($id, bool $useCache = false):array
    {
        $instance = make(get_called_class());
        if ($useCache === true) {
            $modelCache = is_array($id)?$instance->findManyFromCache($id):$instance->findFromCache($id);
            return isset($modelCache) && $modelCache ? $modelCache->toArray() : [];
        }
        $data = $instance->query()->find($id);
        $data || $data = collect([]);
        return $data->toArray();
    }


    /**
     * 根据条件获取结果
     * @param $where
     * @param bool $type 是否查询多条
     * @return array
     */
    public function getInfoByWhere($where, array $columns = ['*'], $type=false)
    {
        $instance = make(get_called_class());

        foreach ($where as $k => $v) {
            $instance = is_array($v)?$instance->where($k,$v[0],$v[1]):$instance->where($k,$v);
        }

        $data = $type?$instance->get($columns):$instance->first($columns);
        $data || $data = collect([]);
        return $data->toArray();
    }

    /**
     * 根据字段及值查找数据信息
     *
     * @param       $field
     * @param       $value
     * @param  array  $columns
     *
     * @return mixed
     */
    public function findOneByField($field, $value = null, $columns = ['*'])
    {
        $instance = make(get_called_class());
        $data = $instance->where($field, '=', $value)->first($columns);
        $data || $data = collect([]);
        return $data->toArray();
    }

    /**
     * 根据条件查找数据
     *
     * @param       $field
     * @param       $value
     * @param  array  $columns
     *
     * @return mixed
     */
    public function findOneWhere(array $where, $columns = ['*'], $order = 'id desc')
    {
        $model = $this->applyConditions($where);
        if ( !empty($order)) {
            $model = $model->orderByRaw($order);
        }
        $data = $model->first($columns);
        $data || $data = collect([]);
        return $data->toArray();
    }

    /**
     * Find data by multiple fields
     *
     * @param array $where
     * @param array $columns
     *
     * @return mixed
     */
    public function findWhere(array $where, $columns = ['*'],$page='',$limit='')
    {
        $model = $this->applyConditions($where);
        if ( !empty($page) || !empty($limit)) {
            $page = $page? $page:0;
            $model = $model->forPage($page, $limit);
        }
        $data = $model->get($columns);
        $data || $data = collect([]);
        return $data->toArray();
    }


    /**
     * 根据id获取数据
     * @param       $id
     * @param  array  $columns
     *
     * @return mixed
     */
    public function findById($id, $columns = ['*'])
    {
        $instance = make(get_called_class());
        $data = $instance->find($id, $columns);
        $data || $data = collect([]);
        return $data->toArray();
    }

    /**
     * 根据id查找数据
     *
     * @param       $id
     * @param  array  $columns
     *
     * @return mixed
     */
    public function findOrFailById($id, $columns = ['*'])
    {
        $instance = make(get_called_class());
        $data = $instance->findOrFail($id, $columns);
        $data || $data = collect([]);
        return $data->toArray();
    }



    /*
      * 获取带分页的查询列表,支持in
      */
    public function getList($where, $page = '', $limit = '', $order = '', $columns = ['*'], $groupBy = '')
    {
        $countNum = $this->countNum($where);
        $where = !is_array($where) ? [] : $where;
        $model = $this->applyConditions($where);
        if ( !empty($page) || !empty($limit)) {
            $page = $page? $page:0;
            $model = $model->forPage($page, $limit);
        }
        if ( !empty($order)) {
            $model = $model->orderByRaw($order);
        }
        if ($groupBy) {
            $result = $model->get($columns)->groupBy($groupBy)->toArray();
        } else {
            $result = $model->get($columns)->toArray();
        }
        if (is_numeric($countNum)) {
            $return = [
                'total' => $countNum,
                'list'  => empty($result) ? [] : $result,
            ];
            if ( !empty($limit)) {
                $return['countpage'] = ceil($countNum / $limit);
            }
            return $return;
        } else {
            return false;
        }
    }

    public function applyConditions(array $where)
    {
//        $this->model = new static();
        $this->model = make(get_called_class())->query();
        foreach ($where as $field => $value) {
            //orWhere条件 $where[] = ['or',[['account_id1','=',$user['id']],['parent_id','=',$user['id']],]]
            if(isset($value[0]) && strtoupper($value[0])=='OR' && is_array($value[1])){
                $orValue = $value[1];
                $this->orWhereCondition($orValue);

            }else if (is_array($value)) {//特殊where条件
                list($field, $condition, $val) = $value;

                $this->whereCondition($field, $condition, $val);
            } else {//默认where条件
                $this->model = $this->model->where($field, '=', $value);
            }
        }
        return $this->model;
    }
    /*
     * orWhere条件支持
     *
     */
    private function orWhereCondition($orValue)
    {
        $this->model->where(function($query) use ($orValue) {
            foreach($orValue as $v){
                list($orfield, $orcondition, $orval) = $v;
                if (strtoupper($orcondition) == 'LOCATE') {
                    $query->orWhereRaw('LOCATE(?,' . $orfield . ') > 0', [$orval]);
                }elseif(strtoupper($orcondition) == 'FIND_IN_SET'){ //字符串中包含某个字符
                    $query->orWhereRaw('FIND_IN_SET(?,' . $orfield . ')', [$orval]);
                }elseif (strtoupper($orcondition) == 'IN') {
                    $query->orwhereIn($orfield, $orval);
                }else{
                    $query->orWhere($orfield,$orcondition,$orval);
                }

            }
        });
    }


    /*
     * 多where条件支持
     */
    private  function whereCondition($field, $condition, $val)
    {
        if (strtoupper($condition) == 'IN') {
            $this->model = $this->model->whereIn($field, $val);
        } elseif (strtoupper($condition) == 'LOCATE') {
            $this->model = $this->model->whereRaw('LOCATE(?,' . $field . ') > 0', [$val]);
        } elseif(strtoupper($condition) == 'REGEXP'){
            $this->model = $this->model->where($field,'REGEXP',$val);
        }elseif(strtoupper($condition) == 'NOT'){
            $this->model = $this->model->whereNotIn($field,$val);
        }elseif(strtoupper($condition) == 'FIND_IN_SET'){ //字符串中包含某个字符
            $this->model = $this->model->whereRaw('FIND_IN_SET(?,' . $field . ')', [$val]);
        }elseif (strtoupper($condition) == 'INSTR') { //字符串中不包含某个字符 instr(goods_id,19);
            $this->model = $this->model->whereRaw('INSTR(' . $field . ',?) =0', [$val]);
        }else{
            $this->model = $this->model->where($field, $condition,$val);
        }
    }

    /**
     * 获取总条数
     * @return mixed
     */
    public function countNum($where = null)
    {
        if ( !empty($where)) {
            $model = $this->applyConditions($where);
        } else {
            $model = make(get_called_class());
        }
        $count = $model->count();
        return $count > 0 ? $count : 0;;
    }

    /**
     * 新增表数据
     */
    public function createDo(array $data)
    {
        if (empty($data)) {
            return false;
        }
        $instance = make(get_called_class());
        $newData = $this->columnsFormat($data, true, true);
        $result = $instance->create($newData);
        return $result;
    }

    /**
     * 修改表数据
     */
    public function updateDo(array $data, $fieldValue = '', $field = 'id')
    {
        if (empty($fieldValue) || empty($data) || empty($field)) {
            return false;
        }
        $instance = make(get_called_class());
        if (is_numeric($fieldValue)) {
            $result = $instance->query(true)->where($field, $fieldValue)->update($data);
        } elseif (is_array($fieldValue)) {
            $result = $instance->query(true)->whereIn($field, $fieldValue)->update($data);
        } else {
            return false;
        }
        return $result;
    }

    /**
     * 根据条件修改表数据
     * @param array $data
     * @param $where
     * @return bool
     */
    public function updateByCondition(array $data, $where)
    {
        if (empty($data) || empty($where)) {
            return false;
        }
        $model = $this->applyConditions($where);
        $newData = $this->columnsFormat($data, true, true);
        $result = $model->update($newData);
        return $result;
    }

    /**
     * 格式化表字段.
     * @param array $value ...
     * @param bool $isTransSnake 是否转snake
     * @param bool $isColumnFilter 是否过滤表不存在的字段
     * @return array ...
     */
    public function columnsFormat(array $value, bool $isTransSnake = false, bool $isColumnFilter = false): array
    {
        $formatValue                     = [];
        $isColumnFilter && $tableColumns = array_flip(\Hyperf\Database\Schema\Schema::getColumnListing($this->getTable()));
        foreach ($value as $field => $fieldValue) {
            ## 转snake
            $isTransSnake && $field = Str::snake($field);
            ## 过滤
            if ($isColumnFilter && ! isset($tableColumns[$field])) {
                continue;
            }
            $formatValue[$field] = $fieldValue;
        }
        return $formatValue;
    }
}
