<?php

namespace App\Core\Common\Handler;

use Monolog\Handler\AbstractProcessingHandler;
use Monolog\Logger;


class LogAliyunHandler extends AbstractProcessingHandler
{


    public function handle(array $record):bool
    {
        if (!$this->isHandling($record)) {
            return false;
        }
        $record = $this->processRecord($record);
        if ( !config('log_enable') ) {
            return false;
        }
        if ( !config('hf_log') && $record['channel'] == 'hyperf' ) {
            return false;
        }
        if ( ! isSupportStdoutLog($record['level_name']) ) {
            return false;
        }
        $record['formatted'] = $this->getFormatter()->format($record);
        $this->write($record);
        return false === $this->bubble;
    }


    /**
     * write
     * 记录日志
     * @param array $record
     * @return bool|void
     */
    public function write(array $record):void
    {
        $saveData = $record['context'];
        $saveData['channel'] = $record['channel'];
        $saveData['message'] = is_array($record['message'])?json_encode($record['message']):$record['message'];
        $saveData['level_name'] = $record['level_name'];
        //todo1

    }

    public function isHandling(array $record):bool
    {
        switch ($record['level']) {
            case Logger::DEBUG:
                return $record['level'] == $this->level;
                break;
            case $record['level'] == Logger::ERROR || $record['level'] == Logger::CRITICAL || $record['level'] == Logger::ALERT || $record['level'] == Logger::EMERGENCY:
                return Logger::ERROR <= $this->level && Logger::EMERGENCY >= $this->level;
                break;
            default:
                return Logger::INFO <= $this->level && Logger::WARNING >= $this->level;
        }
    }

}