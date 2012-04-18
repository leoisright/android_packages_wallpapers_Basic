// Copyright (C) 2009 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#pragma version(1)

#pragma rs java_package_name(com.android.wallpaper.waternerd)

#include "rs_graphics.rsh"

#define LEAVES2_TEXTURES_COUNT 8
#define LEAF_SIZE 0.55f
#define LEAVES2_COUNT 14

// Things we need to set from the application
float g_glWidth;
float g_glHeight;
float g_meshWidth;
float g_meshHeight;
float g_xOffset;
float g_rotate;

rs_program_vertex g_PVWater;
rs_program_vertex g_PVSky2;

rs_program_fragment g_PFSky2;
rs_program_fragment g_PFBackground;

rs_allocation g_TRiverbed;

rs_mesh g_WaterMesh;

typedef struct Constants {
    float4 Drop01;
    float4 Drop02;
    float4 Drop03;
    float4 Drop04;
    float4 Drop05;
    float4 Drop06;
    float4 Drop07;
    float4 Drop08;
    float4 Drop09;
    float4 Drop10;
    float4 Offset;
    float Rotate;
} Constants_t;

Constants_t *g_Constants;
rs_program_store g_PFSBackground;

//float sky2OffsetX;
//float sky2OffsetY;
static float g_DT;
static int64_t g_LastTime;

typedef struct Drop {
    float ampS;
    float ampE;
    float spread;
    float x;
    float y;
} Drop_t;
static Drop_t gDrops[10];
static int gMaxDrops;


void init() {
    int ct;
    gMaxDrops = 10;
    for (ct=0; ct<gMaxDrops; ct++) {
        gDrops[ct].ampS = 0;
        gDrops[ct].ampE = 0;
        gDrops[ct].spread = 1;
    }

}

static void updateDrop(int ct) {
    gDrops[ct].spread += 30.f * g_DT;
    gDrops[ct].ampE = gDrops[ct].ampS / gDrops[ct].spread;
}

static void drop(int x, int y, float s) {
    int ct;
    int iMin = 0;
    float minAmp = 10000.f;
    for (ct = 0; ct < gMaxDrops; ct++) {
        if (gDrops[ct].ampE < minAmp) {
            iMin = ct;
            minAmp = gDrops[ct].ampE;
        }
    }
    gDrops[iMin].ampS = s;
    gDrops[iMin].spread = 0;
    gDrops[iMin].x = x;
    gDrops[iMin].y = g_meshHeight - y - 1;
    updateDrop(iMin);
}

static void generateRipples() {
    int ct;
    for (ct = 0; ct < gMaxDrops; ct++) {
        Drop_t * d = &gDrops[ct];
        float *v = (float*)&g_Constants->Drop01;
        v += ct*4;
        *(v++) = d->x;
        *(v++) = d->y;
        *(v++) = d->ampE * 0.12f;
        *(v++) = d->spread;
    }
    g_Constants->Offset.x = g_xOffset;

    for (ct = 0; ct < gMaxDrops; ct++) {
        updateDrop(ct);
    }
}


static void drawRiverbed() {
    rsgBindProgramFragment(g_PFBackground);
    rsgBindProgramStore(g_PFSBackground);
    rsgBindTexture(g_PFBackground, 0, g_TRiverbed);
    rsgDrawMesh(g_WaterMesh);
}

void addDrop(int x, int y) {
    drop(x, y, 2);
}

int root(void) {
    rsgClearColor(0.f, 0.f, 0.f, 1.f);

    // Compute dt in seconds.
    int64_t newTime = rsUptimeMillis();
    g_DT = (newTime - g_LastTime) * 0.001f;
    g_DT = min(g_DT, 0.2f);
    g_LastTime = newTime;

    g_Constants->Rotate = (float) g_rotate;

    int ct;
    int add = 0;
    for (ct = 0; ct < gMaxDrops; ct++) {
        if (gDrops[ct].ampE < 0.005f) {
            add = 1;
        }
    }

    rsgBindProgramVertex(g_PVWater);
    generateRipples();
    rsgAllocationSyncAll(rsGetAllocation(g_Constants));
    drawRiverbed();

    return 50;
}
