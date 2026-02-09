package com.FerdiStro.drum;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.modes.AbstractMode;
import com.FerdiStro.drum.beat.Beat;
import com.FerdiStro.drum.command.CommandObject;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.memory.SharedMemoryProvider;
import ddf.minim.AudioOutput;
import ddf.minim.Minim;
import ddf.minim.ugens.Delay;
import ddf.minim.ugens.Summer;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.HashMap;
import java.util.Map;

public class DrumMachine {


    protected static final Logger log = LogManager.getLogger();
    private final AbstractMode mode;
    private final Minim minim;
    private final AudioOutput audioOutput;

    private final Summer mixer;


    private final Beat[] bigGrid = new Beat[64];
    private final Beat[] smallGrid = new Beat[8];
    /**
     * Map for faster reading
     */
    private final Map<DrumCommand, CallBack> drumSounds = new HashMap<>();

    public DrumMachine(AbstractMode mode) {
        mode.setDrumMachineCommandLine(this);
        SharedMemoryProvider.getInstance().addMemoryUpdateListeners(mode);

        this.mode = mode;

        this.minim = new Minim();
        this.audioOutput = minim.getLineOut();
        this.mixer = new Summer();

        this.mixer.patch(this.audioOutput);
        drumCommands();
    }


    private Beat getValidBeat(int beatPos) {
        if (mode.isSmallBeat() && beatPos < smallGrid.length) {
            Beat beat = this.smallGrid[beatPos];
            if (beat == null) {
                beat = new Beat(this.minim, this.mixer);
                updateValidBeat(beatPos, beat);
            }
            return beat;
        }
        log.error("Get Big-Beat-Grid: {}", LogUtils.NOT_IMPLEMENTED);
        return null;
    }

    private void updateValidBeat(int beatPos, Beat beat) {

        if (this.mode.isSmallBeat() && beatPos < smallGrid.length) {
            this.smallGrid[beatPos] = beat;
            return;
        }
        log.error("Update Big-Beat-Grid: {}", LogUtils.NOT_IMPLEMENTED);
    }


    private void drumCommands() {
        drumSounds.put(DrumCommand.ADD_SOUND, commandObject -> {
            Beat beat = getValidBeat(commandObject.getBeatPosition());
            beat.addSample(commandObject.getFileName());
            updateValidBeat(commandObject.getBeatPosition(), beat);
        });
    }


    public void onCommand(CommandObject commandObject) {
        log.info("Command {}", commandObject.toString());
        this.drumSounds.get(commandObject.getCommand()).onCallBack(commandObject);
    }


    public void onBeat(int beatPos) {
        Beat beat = getValidBeat(beatPos);

        if (beat != null) {
            try {
                beat.play();

            } catch (Exception e) {
                log.error(e.toString());
                e.printStackTrace();
            }
        }
    }

    interface CallBack {
        void onCallBack(CommandObject commandObject);
    }


}
