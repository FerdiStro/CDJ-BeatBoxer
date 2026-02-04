package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public abstract class AbstractMode {

    public abstract String getName();

    public abstract void startUp();


    protected static final Logger log = LogManager.getLogger();


    public void printStartUpSequence(){
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Start:  {}", getName());
        log.info(LogUtils.LINE_SEPARATOR);
    }


    public void onBeat(){
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("BEAT");
        log.info(LogUtils.LINE_SEPARATOR);
    }

}
