package com.FerdiStro;

import com.FerdiStro.drum.modes.OfflineMode;
import com.FerdiStro.drum.DrumMachine;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.drum.command.DrumCommandObject;
import com.FerdiStro.memory.objects.TransferObject;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestDrumMachine {

    protected static final Logger log = LogManager.getLogger();


    private static final String SAMPLE_KICK = "BeatBoxer-Sounds/KICK_20.wav";

    @Test
    void testDrumMachine() {
        DrumMachine drumMachine = getDrumMachine(1);

        //play
        for (int i = 0; i != 8; i++) {
            drumMachine.onBeat(i);
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                Assertions.fail();
            }
        }

        Assertions.assertTrue(true);
    }

    @Test
    void testTransferObject() {
        TransferObject transferObject = new TransferObject(0.0, (byte) 0, 0, false, getDrumMachine(8).getSmallGrid());
        Assertions.assertEquals(10, transferObject.getTransferBeatData().length);
        Assertions.assertNull(transferObject.getTransferBeatData()[1]);
        // -1 is int 255 and equals 11111111 so beat on each position
        Assertions.assertEquals(-1, transferObject.getTransferBeatData()[0].activeBeatMask);
        transferObject = new TransferObject(0.0, (byte) 0, 0, false, getDrumMachine(4).getSmallGrid());
        Assertions.assertEquals(15, transferObject.getTransferBeatData()[0].activeBeatMask);
        Assertions.assertEquals(0.0, transferObject.getBpm());
        Assertions.assertEquals((byte) 0, transferObject.getSmallCounter());
        Assertions.assertFalse(transferObject.isMaster());

    }

    private DrumMachine getDrumMachine(int beats) {
        OfflineMode offlineMode = new OfflineMode();
        DrumMachine drumMachine = new DrumMachine(offlineMode);
        String totalPath = System.getProperty("user.dir").replace("BeatBoxer-Engien", SAMPLE_KICK);

        //Beats
        for (int i = 0; i != beats; i++) {
            drumMachine.onCommand(new DrumCommandObject(DrumCommand.ADD_SOUND, i, totalPath));
        }

        return drumMachine;
    }
}