package com.FerdiStro.drum;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.modes.AbstractMode;
import com.FerdiStro.drum.beat.Beat;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.drum.command.DrumCommandObject;
import com.FerdiStro.memory.SharedMemoryProvider;
import ddf.minim.AudioOutput;
import ddf.minim.Minim;
import ddf.minim.ugens.Summer;
import lombok.Getter;
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

    @Getter
    private final Beat[] smallGrid = new Beat[8];
    /**
     * Map for faster reading
     */
    private final Map<DrumCommand, CallBack> drumSounds = new HashMap<>();

    public DrumMachine(AbstractMode mode) {
        SharedMemoryProvider.getInstance().addMemoryUpdateListeners(mode);

        this.mode = mode;

        this.minim = new Minim(new MinimHelper());
        this.audioOutput = minim.getLineOut();
        if (this.audioOutput == null) {
            throw new RuntimeException("CRITICAL: No Audio-Output found! (Check PI-Audio)");
        }

        this.mixer = new Summer();

        this.mixer.patch(this.audioOutput);
        drumCommands();
    }


    private Beat getValidBeat(int beatPos) {
        if (mode.isSmallBeat() && beatPos < smallGrid.length) {
            Beat beat = this.smallGrid[beatPos];
            if (beat == null) {
                beat = new Beat(this.minim, this.mixer);
                log.info("Beat_POST: {}", beatPos);
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
            this.mode.sendMemoryUpdate();
            return;
        }
        log.error("Update Big-Beat-Grid: {}", LogUtils.NOT_IMPLEMENTED);
    }


    private void drumCommands() {
        drumSounds.put(DrumCommand.ADD_SOUND, drumCommandObject -> {
            Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
            assert beat != null;
            beat.addSample(drumCommandObject.getFileName());
            updateValidBeat(drumCommandObject.getBeatPosition(), beat);
        });
        drumSounds.put(DrumCommand.REMOVE_SOUND, drumCommandObject -> {
            Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
            assert beat != null;
            beat.removeSample(drumCommandObject.getFileName());
            updateValidBeat(drumCommandObject.getBeatPosition(), beat);
        });
        drumSounds.put(DrumCommand.IGNORE, drumCommandObject -> log.info("Command ignored"));
    }


    public void onCommand(DrumCommandObject drumCommandObject) {
        log.info("Command {}", drumCommandObject.toString());
        this.drumSounds.get(drumCommandObject.getCommand()).onCallBack(drumCommandObject);
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
        void onCallBack(DrumCommandObject drumCommandObject);
    }


}
