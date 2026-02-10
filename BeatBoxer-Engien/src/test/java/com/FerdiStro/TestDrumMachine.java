package com.FerdiStro;

import com.FerdiStro.cdj.modes.OfflineMode;
import com.FerdiStro.drum.DrumMachine;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.drum.command.DrumCommandObject;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestDrumMachine {

    protected static final Logger log = LogManager.getLogger();


    private static final String SAMPLE_KICK = "BeatBoxer-Sounds/KICK_20.wav";

    @Test
    void testDrumMachine() {
        OfflineMode offlineMode = new OfflineMode();
        DrumMachine drumMachine = new DrumMachine(offlineMode);
        String totalPath = System.getProperty("user.dir").replace("BeatBoxer-Engien", SAMPLE_KICK);


        log.info("Sample-Kick path: {} ", totalPath);

        //Beats
        for (int i = 0; i != 7; i++) {
            drumMachine.onCommand(new DrumCommandObject(DrumCommand.ADD_SOUND, i, totalPath));
        }
        //play
        for (int i = 0; i != 7; i++) {
            drumMachine.onBeat(i);
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                Assertions.fail();
            }
        }

        Assertions.assertTrue(true);


    }

}
