package com.FerdiStro.drum.modes.pattern;

import com.FerdiStro.drum.modes.pattern.objects.PatternIndex;
import com.FerdiStro.drum.modes.pattern.objects.PatternMetaData;
import com.FerdiStro.drum.modes.pattern.objects.PatternMetaDataModes;
import com.FerdiStro.drum.modes.pattern.ques.SearchQue;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import tools.jackson.databind.ObjectMapper;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public abstract class AbstractPatternSystem implements Runnable {

    protected static final Logger log = LogManager.getLogger();
    private static final String META_DATA_PATH = "BeatBoxer-Sounds/patterns/pattern_meta_data.json";
    protected final ObjectMapper objectMapper;
    /**
     * Primitive list is used as a return. set changed to false again. Only 10 Patterns are supported in the binary Struct (Binary-Struct.md)
     */
    private final PatternMetaData[] primitiveSearchedPattern = new PatternMetaData[10];
    /**
     * Includes all Patterns. Is loaded form META_DATA_PATH
     */
    protected PatternIndex patternIndexCache;
    /**
     * Search Ques (Recommendation patterns)
     */
    @Setter
    private SearchQue searchQue = null;


    @Getter
    private boolean changed = false;


    private boolean running = true;

    AbstractPatternSystem() {
        this.objectMapper = new ObjectMapper();
        initializeCache();
    }

    /**
     * @return PatternMetaData[] which is the data of the latest search. Set changed status to false again.
     */
    public PatternMetaData[] getSearchedPatternMetaData() {
        this.changed = false;
        return this.primitiveSearchedPattern;
    }

    abstract PatternMetaDataModes getPatternMetaDataMode();


    public void refreshCacheWithPath(String path) {
        File metaFile = new File(path);

        if (!metaFile.exists()) {
            log.error("Meta-Data not found " + path);
            this.patternIndexCache = new PatternIndex(new ArrayList<>());
            return;
        }

        this.patternIndexCache = objectMapper.readValue(metaFile, PatternIndex.class);
    }

    private void initializeCache() {
        refreshCacheWithPath(META_DATA_PATH);
    }

    /**
     * Stop Thread
     */
    public void stop() {
        this.running = false;
    }


    @Override
    public void run() {
        while (running) {
            if (searchQue != null) {
                //max Patterns size is 10 at the moment
                List<PatternMetaData> queList = this.searchPatterns(searchQue);

                for (int i = 0; i != queList.size(); i++) {
                    this.primitiveSearchedPattern[i] = queList.get(i);
                }


                this.changed = true;
                this.searchQue = null;

            }
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }

    }

    /**
     * Implement Mode specific search.
     */
    abstract List<PatternMetaData> specificSearchPattern(SearchQue searchQue);

    private List<PatternMetaData> searchPatterns(SearchQue searchQue) {
        switch (searchQue) {
            case ALL -> {
                return this.patternIndexCache.patterns();
            }
            case MODE -> {
                return this.patternIndexCache.patterns().stream().filter(p -> p.modes().contains(getPatternMetaDataMode())).toList();
            }
        }
        return specificSearchPattern(searchQue);
    }

}
