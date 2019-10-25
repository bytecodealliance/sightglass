
#include <sightglass.h>

#include <assert.h>
#include <stdint.h>
#include <stdlib.h>

#define ITERATIONS 500
#define LENGTH 2500

#ifdef WASM_ENTRY
#ifdef ALT_ENTRY
void
_call(int in)
#else
void
_start(void)
#endif
#else  // WASM_ENTRY
#ifdef EMBED_ENTRY
void
switch2_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    size_t length = LENGTH;

    uint32_t *x;
    x = malloc(LENGTH * sizeof *x);
#ifndef NO_WASI_SUPPORT
    assert(x != NULL);
#endif
    size_t i;
    for (i = (size_t) 0U; i < length; i++) {
        x[i] = i;
    }

    int j;
    for (j = 0; j < ITERATIONS; j++) {
        BLACK_BOX(x);
        BLACK_BOX(length);
        for (i = (size_t) 0U; i < length; i++) {
            switch (x[i]) {
            case 0:
                x[i] ^= 399;

            case 1:
                x[i] ^= 694;

            case 2:
                x[i] ^= 3492;

            case 3:
                x[i] ^= 178;

            case 4:
                x[i] ^= 2502;

            case 5:
                x[i] ^= 3860;

            case 6:
                x[i] ^= 3571;

            case 7:
                x[i] ^= 2405;

            case 8:
                x[i] ^= 1111;

            case 9:
                x[i] ^= 3855;

            case 10:
                x[i] ^= 1320;

            case 11:
                x[i] ^= 60;

            case 12:
                x[i] ^= 3924;

            case 13:
                x[i] ^= 848;

            case 14:
                x[i] ^= 3873;

            case 15:
                x[i] ^= 3852;

            case 16:
                x[i] ^= 1670;

            case 17:
                x[i] ^= 2344;

            case 18:
                x[i] ^= 3258;

            case 19:
                x[i] ^= 1308;

            case 20:
                x[i] ^= 2959;

            case 21:
                x[i] ^= 224;

            case 22:
                x[i] ^= 3613;

            case 23:
                x[i] ^= 2838;

            case 24:
                x[i] ^= 1722;

            case 25:
                x[i] ^= 1429;

            case 26:
                x[i] ^= 3521;

            case 27:
                x[i] ^= 2501;

            case 28:
                x[i] ^= 4006;

            case 29:
                x[i] ^= 2836;

            case 30:
                x[i] ^= 4009;

            case 31:
                x[i] ^= 460;

            case 32:
                x[i] ^= 1458;

            case 33:
                x[i] ^= 862;

            case 34:
                x[i] ^= 1143;

            case 35:
                x[i] ^= 2785;

            case 36:
                x[i] ^= 3637;

            case 37:
                x[i] ^= 3391;

            case 38:
                x[i] ^= 2094;

            case 39:
                x[i] ^= 2534;

            case 40:
                x[i] ^= 474;

            case 41:
                x[i] ^= 2521;

            case 42:
                x[i] ^= 2458;

            case 43:
                x[i] ^= 3871;

            case 44:
                x[i] ^= 400;

            case 45:
                x[i] ^= 3992;

            case 46:
                x[i] ^= 2251;

            case 47:
                x[i] ^= 153;

            case 48:
                x[i] ^= 1280;

            case 49:
                x[i] ^= 341;

            case 50:
                x[i] ^= 3709;

            case 51:
                x[i] ^= 420;

            case 52:
                x[i] ^= 911;

            case 53:
                x[i] ^= 247;

            case 54:
                x[i] ^= 2019;

            case 55:
                x[i] ^= 748;

            case 56:
                x[i] ^= 1510;

            case 57:
                x[i] ^= 1004;

            case 58:
                x[i] ^= 339;

            case 59:
                x[i] ^= 3011;

            case 60:
                x[i] ^= 162;

            case 61:
                x[i] ^= 1042;

            case 62:
                x[i] ^= 2650;

            case 63:
                x[i] ^= 4095;

            case 64:
                x[i] ^= 3883;

            case 65:
                x[i] ^= 1806;

            case 66:
                x[i] ^= 2308;

            case 67:
                x[i] ^= 2721;

            case 68:
                x[i] ^= 725;

            case 69:
                x[i] ^= 3015;

            case 70:
                x[i] ^= 303;

            case 71:
                x[i] ^= 2337;

            case 72:
                x[i] ^= 1381;

            case 73:
                x[i] ^= 1736;

            case 74:
                x[i] ^= 1697;

            case 75:
                x[i] ^= 1022;

            case 76:
                x[i] ^= 986;

            case 77:
                x[i] ^= 900;

            case 78:
                x[i] ^= 1319;

            case 79:
                x[i] ^= 1888;

            case 80:
                x[i] ^= 2474;

            case 81:
                x[i] ^= 3478;

            case 82:
                x[i] ^= 853;

            case 83:
                x[i] ^= 3467;

            case 84:
                x[i] ^= 1639;

            case 85:
                x[i] ^= 2867;

            case 86:
                x[i] ^= 2667;

            case 87:
                x[i] ^= 172;

            case 88:
                x[i] ^= 345;

            case 89:
                x[i] ^= 3386;

            case 90:
                x[i] ^= 2749;

            case 91:
                x[i] ^= 1630;

            case 92:
                x[i] ^= 2719;

            case 93:
                x[i] ^= 3636;

            case 94:
                x[i] ^= 2129;

            case 95:
                x[i] ^= 3470;

            case 96:
                x[i] ^= 271;

            case 97:
                x[i] ^= 158;

            case 98:
                x[i] ^= 1226;

            case 99:
                x[i] ^= 200;

            case 100:
                x[i] ^= 828;

            case 101:
                x[i] ^= 2902;

            case 102:
                x[i] ^= 3740;

            case 103:
                x[i] ^= 446;

            case 104:
                x[i] ^= 3629;

            case 105:
                x[i] ^= 2213;

            case 106:
                x[i] ^= 308;

            case 107:
                x[i] ^= 146;

            case 108:
                x[i] ^= 3998;

            case 109:
                x[i] ^= 1264;

            case 110:
                x[i] ^= 3929;

            case 111:
                x[i] ^= 206;

            case 112:
                x[i] ^= 3293;

            case 113:
                x[i] ^= 2460;

            case 114:
                x[i] ^= 370;

            case 115:
                x[i] ^= 2469;

            case 116:
                x[i] ^= 2196;

            case 117:
                x[i] ^= 2416;

            case 118:
                x[i] ^= 1388;

            case 119:
                x[i] ^= 682;

            case 120:
                x[i] ^= 3192;

            case 121:
                x[i] ^= 207;

            case 122:
                x[i] ^= 1285;

            case 123:
                x[i] ^= 1302;

            case 124:
                x[i] ^= 3181;

            case 125:
                x[i] ^= 2917;

            case 126:
                x[i] ^= 1601;

            case 127:
                x[i] ^= 1502;

            case 128:
                x[i] ^= 2399;

            case 129:
                x[i] ^= 2767;

            case 130:
                x[i] ^= 361;

            case 131:
                x[i] ^= 374;

            case 132:
                x[i] ^= 1650;

            case 133:
                x[i] ^= 3471;

            case 134:
                x[i] ^= 1128;

            case 135:
                x[i] ^= 3441;

            case 136:
                x[i] ^= 868;

            case 137:
                x[i] ^= 2607;

            case 138:
                x[i] ^= 2577;

            case 139:
                x[i] ^= 4012;

            case 140:
                x[i] ^= 981;

            case 141:
                x[i] ^= 1471;

            case 142:
                x[i] ^= 745;

            case 143:
                x[i] ^= 3716;

            case 144:
                x[i] ^= 968;

            case 145:
                x[i] ^= 493;

            case 146:
                x[i] ^= 2374;

            case 147:
                x[i] ^= 2478;

            case 148:
                x[i] ^= 3340;

            case 149:
                x[i] ^= 154;

            case 150:
                x[i] ^= 1347;

            case 151:
                x[i] ^= 3077;

            case 152:
                x[i] ^= 689;

            case 153:
                x[i] ^= 2723;

            case 154:
                x[i] ^= 3691;

            case 155:
                x[i] ^= 3451;

            case 156:
                x[i] ^= 3410;

            case 157:
                x[i] ^= 4056;

            case 158:
                x[i] ^= 2894;

            case 159:
                x[i] ^= 1484;

            case 160:
                x[i] ^= 2000;

            case 161:
                x[i] ^= 1424;

            case 162:
                x[i] ^= 2459;

            case 163:
                x[i] ^= 2260;

            case 164:
                x[i] ^= 3518;

            case 165:
                x[i] ^= 1439;

            case 166:
                x[i] ^= 3527;

            case 167:
                x[i] ^= 188;

            case 168:
                x[i] ^= 3231;

            case 169:
                x[i] ^= 1981;

            case 170:
                x[i] ^= 953;

            case 171:
                x[i] ^= 3872;

            case 172:
                x[i] ^= 2130;

            case 173:
                x[i] ^= 83;

            case 174:
                x[i] ^= 65;

            case 175:
                x[i] ^= 534;

            case 176:
                x[i] ^= 2895;

            case 177:
                x[i] ^= 1433;

            case 178:
                x[i] ^= 1552;

            case 179:
                x[i] ^= 1165;

            case 180:
                x[i] ^= 1229;

            case 181:
                x[i] ^= 1049;

            case 182:
                x[i] ^= 3646;

            case 183:
                x[i] ^= 1544;

            case 184:
                x[i] ^= 1125;

            case 185:
                x[i] ^= 2484;

            case 186:
                x[i] ^= 588;

            case 187:
                x[i] ^= 1528;

            case 188:
                x[i] ^= 1142;

            case 189:
                x[i] ^= 3066;

            case 190:
                x[i] ^= 774;

            case 191:
                x[i] ^= 461;

            case 192:
                x[i] ^= 3870;

            case 193:
                x[i] ^= 1211;

            case 194:
                x[i] ^= 3994;

            case 195:
                x[i] ^= 753;

            case 196:
                x[i] ^= 357;

            case 197:
                x[i] ^= 395;

            case 198:
                x[i] ^= 1682;

            case 199:
                x[i] ^= 829;

            case 200:
                x[i] ^= 3089;

            case 201:
                x[i] ^= 3222;

            case 202:
                x[i] ^= 3975;

            case 203:
                x[i] ^= 1731;

            case 204:
                x[i] ^= 3415;

            case 205:
                x[i] ^= 2430;

            case 206:
                x[i] ^= 1213;

            case 207:
                x[i] ^= 4003;

            case 208:
                x[i] ^= 784;

            case 209:
                x[i] ^= 253;

            case 210:
                x[i] ^= 3624;

            case 211:
                x[i] ^= 898;

            case 212:
                x[i] ^= 3433;

            case 213:
                x[i] ^= 2745;

            case 214:
                x[i] ^= 323;

            case 215:
                x[i] ^= 764;

            case 216:
                x[i] ^= 2199;

            case 217:
                x[i] ^= 3360;

            case 218:
                x[i] ^= 2209;

            case 219:
                x[i] ^= 1322;

            case 220:
                x[i] ^= 1426;

            case 221:
                x[i] ^= 3824;

            case 222:
                x[i] ^= 141;

            case 223:
                x[i] ^= 1556;

            case 224:
                x[i] ^= 2860;

            case 225:
                x[i] ^= 3777;

            case 226:
                x[i] ^= 1029;

            case 227:
                x[i] ^= 1073;

            case 228:
                x[i] ^= 874;

            case 229:
                x[i] ^= 883;

            case 230:
                x[i] ^= 107;

            case 231:
                x[i] ^= 3877;

            case 232:
                x[i] ^= 313;

            case 233:
                x[i] ^= 354;

            case 234:
                x[i] ^= 1956;

            case 235:
                x[i] ^= 1677;

            case 236:
                x[i] ^= 545;

            case 237:
                x[i] ^= 888;

            case 238:
                x[i] ^= 1704;

            case 239:
                x[i] ^= 3369;

            case 240:
                x[i] ^= 2291;

            case 241:
                x[i] ^= 3508;

            case 242:
                x[i] ^= 3878;

            case 243:
                x[i] ^= 382;

            case 244:
                x[i] ^= 3822;

            case 245:
                x[i] ^= 3134;

            case 246:
                x[i] ^= 3903;

            case 247:
                x[i] ^= 844;

            case 248:
                x[i] ^= 3603;

            case 249:
                x[i] ^= 3816;

            case 250:
                x[i] ^= 2908;

            case 251:
                x[i] ^= 2833;

            case 252:
                x[i] ^= 2266;

            case 253:
                x[i] ^= 3724;

            case 254:
                x[i] ^= 2532;

            case 255:
                x[i] ^= 307;

            case 256:
                x[i] ^= 2777;

            case 257:
                x[i] ^= 239;

            case 258:
                x[i] ^= 2033;

            case 259:
                x[i] ^= 1178;

            case 260:
                x[i] ^= 2073;

            case 261:
                x[i] ^= 1436;

            case 262:
                x[i] ^= 1334;

            case 263:
                x[i] ^= 2855;

            case 264:
                x[i] ^= 2877;

            case 265:
                x[i] ^= 2372;

            case 266:
                x[i] ^= 2553;

            case 267:
                x[i] ^= 2885;

            case 268:
                x[i] ^= 2133;

            case 269:
                x[i] ^= 2470;

            case 270:
                x[i] ^= 91;

            case 271:
                x[i] ^= 4091;

            case 272:
                x[i] ^= 2519;

            case 273:
                x[i] ^= 1879;

            case 274:
                x[i] ^= 3074;

            case 275:
                x[i] ^= 1541;

            case 276:
                x[i] ^= 3164;

            case 277:
                x[i] ^= 150;

            case 278:
                x[i] ^= 394;

            case 279:
                x[i] ^= 1546;

            case 280:
                x[i] ^= 3807;

            case 281:
                x[i] ^= 3918;

            case 282:
                x[i] ^= 1564;

            case 283:
                x[i] ^= 3599;

            case 284:
                x[i] ^= 1707;

            case 285:
                x[i] ^= 2115;

            case 286:
                x[i] ^= 2006;

            case 287:
                x[i] ^= 1304;

            case 288:
                x[i] ^= 982;

            case 289:
                x[i] ^= 3573;

            case 290:
                x[i] ^= 5;

            case 291:
                x[i] ^= 664;

            case 292:
                x[i] ^= 1481;

            case 293:
                x[i] ^= 785;

            case 294:
                x[i] ^= 3546;

            case 295:
                x[i] ^= 1873;

            case 296:
                x[i] ^= 3868;

            case 297:
                x[i] ^= 67;

            case 298:
                x[i] ^= 2956;

            case 299:
                x[i] ^= 3259;

            case 300:
                x[i] ^= 3848;

            case 301:
                x[i] ^= 2933;

            case 302:
                x[i] ^= 1713;

            case 303:
                x[i] ^= 2578;

            case 304:
                x[i] ^= 584;

            case 305:
                x[i] ^= 1766;

            case 306:
                x[i] ^= 144;

            case 307:
                x[i] ^= 660;

            case 308:
                x[i] ^= 2105;

            case 309:
                x[i] ^= 3605;

            case 310:
                x[i] ^= 667;

            case 311:
                x[i] ^= 723;

            case 312:
                x[i] ^= 3820;

            case 313:
                x[i] ^= 3346;

            case 314:
                x[i] ^= 1329;

            case 315:
                x[i] ^= 1841;

            case 316:
                x[i] ^= 3238;

            case 317:
                x[i] ^= 3796;

            case 318:
                x[i] ^= 2275;

            case 319:
                x[i] ^= 904;

            case 320:
                x[i] ^= 703;

            case 321:
                x[i] ^= 283;

            case 322:
                x[i] ^= 2321;

            case 323:
                x[i] ^= 536;

            case 324:
                x[i] ^= 2725;

            case 325:
                x[i] ^= 877;

            case 326:
                x[i] ^= 3177;

            case 327:
                x[i] ^= 935;

            case 328:
                x[i] ^= 780;

            case 329:
                x[i] ^= 1902;

            case 330:
                x[i] ^= 697;

            case 331:
                x[i] ^= 21;

            case 332:
                x[i] ^= 1706;

            case 333:
                x[i] ^= 1592;

            case 334:
                x[i] ^= 349;

            case 335:
                x[i] ^= 943;

            case 336:
                x[i] ^= 2016;

            case 337:
                x[i] ^= 3768;

            case 338:
                x[i] ^= 1529;

            case 339:
                x[i] ^= 560;

            case 340:
                x[i] ^= 3084;

            case 341:
                x[i] ^= 2678;

            case 342:
                x[i] ^= 1523;

            case 343:
                x[i] ^= 1089;

            case 344:
                x[i] ^= 2954;

            case 345:
                x[i] ^= 2522;

            case 346:
                x[i] ^= 33;

            case 347:
                x[i] ^= 1244;

            case 348:
                x[i] ^= 1811;

            case 349:
                x[i] ^= 1580;

            case 350:
                x[i] ^= 2025;

            case 351:
                x[i] ^= 1274;

            case 352:
                x[i] ^= 2794;

            case 353:
                x[i] ^= 2883;

            case 354:
                x[i] ^= 2254;

            case 355:
                x[i] ^= 3121;

            case 356:
                x[i] ^= 1207;

            case 357:
                x[i] ^= 655;

            case 358:
                x[i] ^= 2595;

            case 359:
                x[i] ^= 1130;

            case 360:
                x[i] ^= 417;

            case 361:
                x[i] ^= 2261;

            case 362:
                x[i] ^= 1715;

            case 363:
                x[i] ^= 434;

            case 364:
                x[i] ^= 359;

            case 365:
                x[i] ^= 53;

            case 366:
                x[i] ^= 2055;

            case 367:
                x[i] ^= 1032;

            case 368:
                x[i] ^= 1053;

            case 369:
                x[i] ^= 1641;

            case 370:
                x[i] ^= 2654;

            case 371:
                x[i] ^= 433;

            case 372:
                x[i] ^= 1794;

            case 373:
                x[i] ^= 3440;

            case 374:
                x[i] ^= 23;

            case 375:
                x[i] ^= 4;

            case 376:
                x[i] ^= 2793;

            case 377:
                x[i] ^= 3733;

            case 378:
                x[i] ^= 3210;

            case 379:
                x[i] ^= 3662;

            case 380:
                x[i] ^= 3301;

            case 381:
                x[i] ^= 1396;

            case 382:
                x[i] ^= 1760;

            case 383:
                x[i] ^= 3862;

            case 384:
                x[i] ^= 3993;

            case 385:
                x[i] ^= 1748;

            case 386:
                x[i] ^= 3109;

            case 387:
                x[i] ^= 2239;

            case 388:
                x[i] ^= 2715;

            case 389:
                x[i] ^= 527;

            case 390:
                x[i] ^= 3239;

            case 391:
                x[i] ^= 587;

            case 392:
                x[i] ^= 3092;

            case 393:
                x[i] ^= 191;

            case 394:
                x[i] ^= 2435;

            case 395:
                x[i] ^= 3818;

            case 396:
                x[i] ^= 3826;

            case 397:
                x[i] ^= 2231;

            case 398:
                x[i] ^= 3935;

            case 399:
                x[i] ^= 1208;

            case 400:
                x[i] ^= 2018;

            case 401:
                x[i] ^= 840;

            case 402:
                x[i] ^= 3248;

            case 403:
                x[i] ^= 2763;

            case 404:
                x[i] ^= 3431;

            case 405:
                x[i] ^= 2846;

            case 406:
                x[i] ^= 3264;

            case 407:
                x[i] ^= 959;

            case 408:
                x[i] ^= 2143;

            case 409:
                x[i] ^= 2835;

            case 410:
                x[i] ^= 25;

            case 411:
                x[i] ^= 3885;

            case 412:
                x[i] ^= 1189;

            case 413:
                x[i] ^= 855;

            case 414:
                x[i] ^= 533;

            case 415:
                x[i] ^= 3183;

            case 416:
                x[i] ^= 2396;

            case 417:
                x[i] ^= 164;

            case 418:
                x[i] ^= 1574;

            case 419:
                x[i] ^= 4021;

            case 420:
                x[i] ^= 1013;

            case 421:
                x[i] ^= 2991;

            case 422:
                x[i] ^= 1187;

            case 423:
                x[i] ^= 936;

            case 424:
                x[i] ^= 1441;

            case 425:
                x[i] ^= 3132;

            case 426:
                x[i] ^= 3817;

            case 427:
                x[i] ^= 226;

            case 428:
                x[i] ^= 4055;

            case 429:
                x[i] ^= 3568;

            case 430:
                x[i] ^= 2634;

            case 431:
                x[i] ^= 1238;

            case 432:
                x[i] ^= 1016;

            case 433:
                x[i] ^= 2172;

            case 434:
                x[i] ^= 1719;

            case 435:
                x[i] ^= 1509;

            case 436:
                x[i] ^= 3178;

            case 437:
                x[i] ^= 2909;

            case 438:
                x[i] ^= 1634;

            case 439:
                x[i] ^= 142;

            case 440:
                x[i] ^= 752;

            case 441:
                x[i] ^= 2361;

            case 442:
                x[i] ^= 3114;

            case 443:
                x[i] ^= 3526;

            case 444:
                x[i] ^= 3296;

            case 445:
                x[i] ^= 1074;

            case 446:
                x[i] ^= 1062;

            case 447:
                x[i] ^= 119;

            case 448:
                x[i] ^= 2628;

            case 449:
                x[i] ^= 1965;

            case 450:
                x[i] ^= 2612;

            case 451:
                x[i] ^= 2310;

            case 452:
                x[i] ^= 2904;

            case 453:
                x[i] ^= 3275;

            case 454:
                x[i] ^= 1339;

            case 455:
                x[i] ^= 3358;

            case 456:
                x[i] ^= 2791;

            case 457:
                x[i] ^= 12;

            case 458:
                x[i] ^= 371;

            case 459:
                x[i] ^= 2038;

            case 460:
                x[i] ^= 301;

            case 461:
                x[i] ^= 476;

            case 462:
                x[i] ^= 1363;

            case 463:
                x[i] ^= 2084;

            case 464:
                x[i] ^= 677;

            case 465:
                x[i] ^= 1366;

            case 466:
                x[i] ^= 1955;

            case 467:
                x[i] ^= 269;

            case 468:
                x[i] ^= 972;

            case 469:
                x[i] ^= 2798;

            case 470:
                x[i] ^= 1406;

            case 471:
                x[i] ^= 4058;

            case 472:
                x[i] ^= 2948;

            case 473:
                x[i] ^= 3250;

            case 474:
                x[i] ^= 1083;

            case 475:
                x[i] ^= 1496;

            case 476:
                x[i] ^= 1847;

            case 477:
                x[i] ^= 3988;

            case 478:
                x[i] ^= 699;

            case 479:
                x[i] ^= 2982;

            case 480:
                x[i] ^= 1278;

            case 481:
                x[i] ^= 3180;

            case 482:
                x[i] ^= 1910;

            case 483:
                x[i] ^= 800;

            case 484:
                x[i] ^= 766;

            case 485:
                x[i] ^= 3723;

            case 486:
                x[i] ^= 18;

            case 487:
                x[i] ^= 3616;

            case 488:
                x[i] ^= 3350;

            case 489:
                x[i] ^= 2708;

            case 490:
                x[i] ^= 3727;

            case 491:
                x[i] ^= 377;

            case 492:
                x[i] ^= 3007;

            case 493:
                x[i] ^= 2114;

            case 494:
                x[i] ^= 1116;

            case 495:
                x[i] ^= 3006;

            case 496:
                x[i] ^= 1762;

            case 497:
                x[i] ^= 3395;

            case 498:
                x[i] ^= 1796;

            case 499:
                x[i] ^= 2176;

            case 500:
                x[i] ^= 2227;

            case 501:
                x[i] ^= 487;

            case 502:
                x[i] ^= 2778;

            case 503:
                x[i] ^= 648;

            case 504:
                x[i] ^= 3717;

            case 505:
                x[i] ^= 1199;

            case 506:
                x[i] ^= 3653;

            case 507:
                x[i] ^= 3682;

            case 508:
                x[i] ^= 500;

            case 509:
                x[i] ^= 2845;

            case 510:
                x[i] ^= 1194;

            case 511:
                x[i] ^= 586;

            case 512:
                x[i] ^= 3274;

            case 513:
                x[i] ^= 229;

            case 514:
                x[i] ^= 1382;

            case 515:
                x[i] ^= 100;

            case 516:
                x[i] ^= 891;

            case 517:
                x[i] ^= 3562;

            case 518:
                x[i] ^= 1287;

            case 519:
                x[i] ^= 1031;

            case 520:
                x[i] ^= 2660;

            case 521:
                x[i] ^= 1856;

            case 522:
                x[i] ^= 1652;

            case 523:
                x[i] ^= 3908;

            case 524:
                x[i] ^= 3247;

            case 525:
                x[i] ^= 1474;

            case 526:
                x[i] ^= 3945;

            case 527:
                x[i] ^= 1352;

            case 528:
                x[i] ^= 3030;

            case 529:
                x[i] ^= 2288;

            case 530:
                x[i] ^= 3080;

            case 531:
                x[i] ^= 438;

            case 532:
                x[i] ^= 1046;

            case 533:
                x[i] ^= 1033;

            case 534:
                x[i] ^= 1328;

            case 535:
                x[i] ^= 1572;

            case 536:
                x[i] ^= 3188;

            case 537:
                x[i] ^= 3145;

            case 538:
                x[i] ^= 2299;

            case 539:
                x[i] ^= 1831;

            case 540:
                x[i] ^= 771;

            case 541:
                x[i] ^= 3363;

            case 542:
                x[i] ^= 338;

            case 543:
                x[i] ^= 3537;

            case 544:
                x[i] ^= 2726;

            case 545:
                x[i] ^= 956;

            case 546:
                x[i] ^= 905;

            case 547:
                x[i] ^= 2249;

            case 548:
                x[i] ^= 1218;

            case 549:
                x[i] ^= 3936;

            case 550:
                x[i] ^= 672;

            case 551:
                x[i] ^= 2687;

            case 552:
                x[i] ^= 3052;

            case 553:
                x[i] ^= 676;

            case 554:
                x[i] ^= 504;

            case 555:
                x[i] ^= 3213;

            case 556:
                x[i] ^= 826;

            case 557:
                x[i] ^= 346;

            case 558:
                x[i] ^= 944;

            case 559:
                x[i] ^= 1566;

            case 560:
                x[i] ^= 2340;

            case 561:
                x[i] ^= 566;

            case 562:
                x[i] ^= 749;

            case 563:
                x[i] ^= 3223;

            case 564:
                x[i] ^= 386;

            case 565:
                x[i] ^= 2523;

            case 566:
                x[i] ^= 3037;

            case 567:
                x[i] ^= 837;

            case 568:
                x[i] ^= 1499;

            case 569:
                x[i] ^= 246;

            case 570:
                x[i] ^= 2184;

            case 571:
                x[i] ^= 1072;

            case 572:
                x[i] ^= 3695;

            case 573:
                x[i] ^= 97;

            case 574:
                x[i] ^= 3749;

            case 575:
                x[i] ^= 1144;

            case 576:
                x[i] ^= 2622;

            case 577:
                x[i] ^= 2408;

            case 578:
                x[i] ^= 831;

            case 579:
                x[i] ^= 1994;

            case 580:
                x[i] ^= 1468;

            case 581:
                x[i] ^= 3434;

            case 582:
                x[i] ^= 1030;

            case 583:
                x[i] ^= 3766;

            case 584:
                x[i] ^= 3970;

            case 585:
                x[i] ^= 856;

            case 586:
                x[i] ^= 2582;

            case 587:
                x[i] ^= 1267;

            case 588:
                x[i] ^= 3510;

            case 589:
                x[i] ^= 3655;

            case 590:
                x[i] ^= 2142;

            case 591:
                x[i] ^= 925;

            case 592:
                x[i] ^= 3774;

            case 593:
                x[i] ^= 1336;

            case 594:
                x[i] ^= 3477;

            case 595:
                x[i] ^= 3672;

            case 596:
                x[i] ^= 3422;

            case 597:
                x[i] ^= 561;

            case 598:
                x[i] ^= 512;

            case 599:
                x[i] ^= 3640;

            case 600:
                x[i] ^= 1683;

            case 601:
                x[i] ^= 3594;

            case 602:
                x[i] ^= 372;

            case 603:
                x[i] ^= 2162;

            case 604:
                x[i] ^= 2916;

            case 605:
                x[i] ^= 942;

            case 606:
                x[i] ^= 2507;

            case 607:
                x[i] ^= 1003;

            case 608:
                x[i] ^= 2743;

            case 609:
                x[i] ^= 2393;

            case 610:
                x[i] ^= 3067;

            case 611:
                x[i] ^= 459;

            case 612:
                x[i] ^= 3619;

            case 613:
                x[i] ^= 1250;

            case 614:
                x[i] ^= 1311;

            case 615:
                x[i] ^= 934;

            case 616:
                x[i] ^= 1277;

            case 617:
                x[i] ^= 4049;

            case 618:
                x[i] ^= 2990;

            case 619:
                x[i] ^= 118;

            case 620:
                x[i] ^= 2269;

            case 621:
                x[i] ^= 1297;

            case 622:
                x[i] ^= 3;

            case 623:
                x[i] ^= 1038;

            case 624:
                x[i] ^= 289;

            case 625:
                x[i] ^= 1885;

            case 626:
                x[i] ^= 2366;

            case 627:
                x[i] ^= 3423;

            case 628:
                x[i] ^= 3076;

            case 629:
                x[i] ^= 3327;

            case 630:
                x[i] ^= 1620;

            case 631:
                x[i] ^= 3829;

            case 632:
                x[i] ^= 1973;

            case 633:
                x[i] ^= 902;

            case 634:
                x[i] ^= 3425;

            case 635:
                x[i] ^= 583;

            case 636:
                x[i] ^= 632;

            case 637:
                x[i] ^= 2563;

            case 638:
                x[i] ^= 2913;

            case 639:
                x[i] ^= 3858;

            case 640:
                x[i] ^= 1056;

            case 641:
                x[i] ^= 1052;

            case 642:
                x[i] ^= 1567;

            case 643:
                x[i] ^= 653;

            case 644:
                x[i] ^= 3692;

            case 645:
                x[i] ^= 3311;

            case 646:
                x[i] ^= 1616;

            case 647:
                x[i] ^= 1989;

            case 648:
                x[i] ^= 963;

            case 649:
                x[i] ^= 3396;

            case 650:
                x[i] ^= 1978;

            case 651:
                x[i] ^= 591;

            case 652:
                x[i] ^= 2216;

            case 653:
                x[i] ^= 924;

            case 654:
                x[i] ^= 194;

            case 655:
                x[i] ^= 732;

            case 656:
                x[i] ^= 692;

            case 657:
                x[i] ^= 2500;

            case 658:
                x[i] ^= 2336;

            case 659:
                x[i] ^= 615;

            case 660:
                x[i] ^= 685;

            case 661:
                x[i] ^= 41;

            case 662:
                x[i] ^= 3202;

            case 663:
                x[i] ^= 553;

            case 664:
                x[i] ^= 2976;

            case 665:
                x[i] ^= 1084;

            case 666:
                x[i] ^= 1767;

            case 667:
                x[i] ^= 3759;

            case 668:
                x[i] ^= 1576;

            case 669:
                x[i] ^= 3493;

            case 670:
                x[i] ^= 27;

            case 671:
                x[i] ^= 1657;

            case 672:
                x[i] ^= 3257;

            case 673:
                x[i] ^= 3126;

            case 674:
                x[i] ^= 3623;

            case 675:
                x[i] ^= 3742;

            case 676:
                x[i] ^= 552;

            case 677:
                x[i] ^= 1294;

            case 678:
                x[i] ^= 1983;

            case 679:
                x[i] ^= 726;

            case 680:
                x[i] ^= 1797;

            case 681:
                x[i] ^= 554;

            case 682:
                x[i] ^= 3867;

            case 683:
                x[i] ^= 2978;

            case 684:
                x[i] ^= 2697;

            case 685:
                x[i] ^= 2429;

            case 686:
                x[i] ^= 3788;

            case 687:
                x[i] ^= 618;

            case 688:
                x[i] ^= 3056;

            case 689:
                x[i] ^= 2963;

            case 690:
                x[i] ^= 1303;

            case 691:
                x[i] ^= 2889;

            case 692:
                x[i] ^= 2072;

            case 693:
                x[i] ^= 4051;

            case 694:
                x[i] ^= 2456;

            case 695:
                x[i] ^= 3853;

            case 696:
                x[i] ^= 1765;

            case 697:
                x[i] ^= 2050;

            case 698:
                x[i] ^= 34;

            case 699:
                x[i] ^= 511;

            case 700:
                x[i] ^= 294;

            case 701:
                x[i] ^= 3203;

            case 702:
                x[i] ^= 696;

            case 703:
                x[i] ^= 3548;

            case 704:
                x[i] ^= 181;

            case 705:
                x[i] ^= 913;

            case 706:
                x[i] ^= 3751;

            case 707:
                x[i] ^= 1249;

            case 708:
                x[i] ^= 2748;

            case 709:
                x[i] ^= 2293;

            case 710:
                x[i] ^= 3525;

            case 711:
                x[i] ^= 3802;

            case 712:
                x[i] ^= 3379;

            case 713:
                x[i] ^= 3149;

            case 714:
                x[i] ^= 3394;

            case 715:
                x[i] ^= 3834;

            case 716:
                x[i] ^= 2641;

            case 717:
                x[i] ^= 2606;

            case 718:
                x[i] ^= 686;

            case 719:
                x[i] ^= 281;

            case 720:
                x[i] ^= 1539;

            case 721:
                x[i] ^= 276;

            case 722:
                x[i] ^= 989;

            case 723:
                x[i] ^= 275;

            case 724:
                x[i] ^= 3404;

            case 725:
                x[i] ^= 782;

            case 726:
                x[i] ^= 1750;

            case 727:
                x[i] ^= 55;

            case 728:
                x[i] ^= 980;

            case 729:
                x[i] ^= 2938;

            case 730:
                x[i] ^= 2882;

            case 731:
                x[i] ^= 1941;

            case 732:
                x[i] ^= 1864;

            case 733:
                x[i] ^= 2301;

            case 734:
                x[i] ^= 3940;

            case 735:
                x[i] ^= 2303;

            case 736:
                x[i] ^= 1070;

            case 737:
                x[i] ^= 3462;

            case 738:
                x[i] ^= 2;

            case 739:
                x[i] ^= 3138;

            case 740:
                x[i] ^= 2579;

            case 741:
                x[i] ^= 3847;

            case 742:
                x[i] ^= 2248;

            case 743:
                x[i] ^= 3764;

            case 744:
                x[i] ^= 2292;

            case 745:
                x[i] ^= 2304;

            case 746:
                x[i] ^= 116;

            case 747:
                x[i] ^= 193;

            case 748:
                x[i] ^= 1041;

            case 749:
                x[i] ^= 1754;

            case 750:
                x[i] ^= 72;

            case 751:
                x[i] ^= 3333;

            case 752:
                x[i] ^= 1963;

            case 753:
                x[i] ^= 1774;

            case 754:
                x[i] ^= 3411;

            case 755:
                x[i] ^= 2148;

            case 756:
                x[i] ^= 3738;

            case 757:
                x[i] ^= 3980;

            case 758:
                x[i] ^= 2141;

            case 759:
                x[i] ^= 3199;

            case 760:
                x[i] ^= 2448;

            case 761:
                x[i] ^= 2046;

            case 762:
                x[i] ^= 1168;

            case 763:
                x[i] ^= 3053;

            case 764:
                x[i] ^= 1245;

            case 765:
                x[i] ^= 3514;

            case 766:
                x[i] ^= 1977;

            case 767:
                x[i] ^= 492;

            case 768:
                x[i] ^= 2388;

            case 769:
                x[i] ^= 585;

            case 770:
                x[i] ^= 1503;

            case 771:
                x[i] ^= 3699;

            case 772:
                x[i] ^= 368;

            case 773:
                x[i] ^= 3658;

            case 774:
                x[i] ^= 1562;

            case 775:
                x[i] ^= 108;

            case 776:
                x[i] ^= 3353;

            case 777:
                x[i] ^= 1024;

            case 778:
                x[i] ^= 1260;

            case 779:
                x[i] ^= 2495;

            case 780:
                x[i] ^= 1871;

            case 781:
                x[i] ^= 3044;

            case 782:
                x[i] ^= 2940;

            case 783:
                x[i] ^= 1729;

            case 784:
                x[i] ^= 1824;

            case 785:
                x[i] ^= 2510;

            case 786:
                x[i] ^= 1755;

            case 787:
                x[i] ^= 3926;

            case 788:
                x[i] ^= 2876;

            case 789:
                x[i] ^= 1449;

            case 790:
                x[i] ^= 2526;

            case 791:
                x[i] ^= 2923;

            case 792:
                x[i] ^= 559;

            case 793:
                x[i] ^= 622;

            case 794:
                x[i] ^= 198;

            case 795:
                x[i] ^= 3292;

            case 796:
                x[i] ^= 320;

            case 797:
                x[i] ^= 2571;

            case 798:
                x[i] ^= 1663;

            case 799:
                x[i] ^= 2556;

            case 800:
                x[i] ^= 1753;

            case 801:
                x[i] ^= 2906;

            case 802:
                x[i] ^= 2900;

            case 803:
                x[i] ^= 544;

            case 804:
                x[i] ^= 638;

            case 805:
                x[i] ^= 2004;

            case 806:
                x[i] ^= 2413;

            case 807:
                x[i] ^= 13;

            case 808:
                x[i] ^= 2345;

            case 809:
                x[i] ^= 280;

            case 810:
                x[i] ^= 376;

            case 811:
                x[i] ^= 746;

            case 812:
                x[i] ^= 485;

            case 813:
                x[i] ^= 393;

            case 814:
                x[i] ^= 3317;

            case 815:
                x[i] ^= 975;

            case 816:
                x[i] ^= 437;

            case 817:
                x[i] ^= 4044;

            case 818:
                x[i] ^= 270;

            case 819:
                x[i] ^= 4067;

            case 820:
                x[i] ^= 3457;

            case 821:
                x[i] ^= 2648;

            case 822:
                x[i] ^= 1865;

            case 823:
                x[i] ^= 3450;

            case 824:
                x[i] ^= 2984;

            case 825:
                x[i] ^= 4064;

            case 826:
                x[i] ^= 1134;

            case 827:
                x[i] ^= 2947;

            case 828:
                x[i] ^= 344;

            case 829:
                x[i] ^= 3720;

            case 830:
                x[i] ^= 3713;

            case 831:
                x[i] ^= 1039;

            case 832:
                x[i] ^= 3357;

            case 833:
                x[i] ^= 870;

            case 834:
                x[i] ^= 2265;

            case 835:
                x[i] ^= 2854;

            case 836:
                x[i] ^= 2574;

            case 837:
                x[i] ^= 3045;

            case 838:
                x[i] ^= 1579;

            case 839:
                x[i] ^= 530;

            case 840:
                x[i] ^= 3430;

            case 841:
                x[i] ^= 1430;

            case 842:
                x[i] ^= 3718;

            case 843:
                x[i] ^= 2367;

            case 844:
                x[i] ^= 602;

            case 845:
                x[i] ^= 919;

            case 846:
                x[i] ^= 575;

            case 847:
                x[i] ^= 2281;

            case 848:
                x[i] ^= 835;

            case 849:
                x[i] ^= 3906;

            case 850:
                x[i] ^= 773;

            case 851:
                x[i] ^= 3278;

            case 852:
                x[i] ^= 3499;

            case 853:
                x[i] ^= 1196;

            case 854:
                x[i] ^= 3204;

            case 855:
                x[i] ^= 1773;

            case 856:
                x[i] ^= 1390;

            case 857:
                x[i] ^= 2572;

            case 858:
                x[i] ^= 971;

            case 859:
                x[i] ^= 2202;

            case 860:
                x[i] ^= 4080;

            case 861:
                x[i] ^= 2768;

            case 862:
                x[i] ^= 2928;

            case 863:
                x[i] ^= 3809;

            case 864:
                x[i] ^= 3681;

            case 865:
                x[i] ^= 3103;

            case 866:
                x[i] ^= 3976;

            case 867:
                x[i] ^= 564;

            case 868:
                x[i] ^= 3520;

            case 869:
                x[i] ^= 3748;

            case 870:
                x[i] ^= 1658;

            case 871:
                x[i] ^= 2827;

            case 872:
                x[i] ^= 3879;

            case 873:
                x[i] ^= 3598;

            case 874:
                x[i] ^= 2338;

            case 875:
                x[i] ^= 1275;

            case 876:
                x[i] ^= 3476;

            case 877:
                x[i] ^= 1550;

            case 878:
                x[i] ^= 24;

            case 879:
                x[i] ^= 1547;

            case 880:
                x[i] ^= 1040;

            case 881:
                x[i] ^= 2225;

            case 882:
                x[i] ^= 1220;

            case 883:
                x[i] ^= 521;

            case 884:
                x[i] ^= 1604;

            case 885:
                x[i] ^= 1459;

            case 886:
                x[i] ^= 1360;

            case 887:
                x[i] ^= 264;

            case 888:
                x[i] ^= 4069;

            case 889:
                x[i] ^= 2068;

            case 890:
                x[i] ^= 3316;

            case 891:
                x[i] ^= 1958;

            case 892:
                x[i] ^= 1870;

            case 893:
                x[i] ^= 2540;

            case 894:
                x[i] ^= 1300;

            case 895:
                x[i] ^= 1959;

            case 896:
                x[i] ^= 261;

            case 897:
                x[i] ^= 387;

            case 898:
                x[i] ^= 941;

            case 899:
                x[i] ^= 2480;

            case 900:
                x[i] ^= 3657;

            case 901:
                x[i] ^= 2193;

            case 902:
                x[i] ^= 80;

            case 903:
                x[i] ^= 227;

            case 904:
                x[i] ^= 2741;

            case 905:
                x[i] ^= 1814;

            case 906:
                x[i] ^= 3479;

            case 907:
                x[i] ^= 486;

            case 908:
                x[i] ^= 3721;

            case 909:
                x[i] ^= 1781;

            case 910:
                x[i] ^= 2664;

            case 911:
                x[i] ^= 2360;

            case 912:
                x[i] ^= 767;

            case 913:
                x[i] ^= 1625;

            case 914:
                x[i] ^= 3512;

            case 915:
                x[i] ^= 1944;

            case 916:
                x[i] ^= 436;

            case 917:
                x[i] ^= 3153;

            case 918:
                x[i] ^= 3968;

            case 919:
                x[i] ^= 3990;

            case 920:
                x[i] ^= 2097;

            case 921:
                x[i] ^= 2869;

            case 922:
                x[i] ^= 3446;

            case 923:
                x[i] ^= 1365;

            case 924:
                x[i] ^= 3719;

            case 925:
                x[i] ^= 1585;

            case 926:
                x[i] ^= 845;

            case 927:
                x[i] ^= 994;

            case 928:
                x[i] ^= 306;

            case 929:
                x[i] ^= 2653;

            case 930:
                x[i] ^= 2599;

            case 931:
                x[i] ^= 548;

            case 932:
                x[i] ^= 2554;

            case 933:
                x[i] ^= 640;

            case 934:
                x[i] ^= 3179;

            case 935:
                x[i] ^= 627;

            case 936:
                x[i] ^= 2362;

            case 937:
                x[i] ^= 2243;

            case 938:
                x[i] ^= 720;

            case 939:
                x[i] ^= 1408;

            case 940:
                x[i] ^= 4035;

            case 941:
                x[i] ^= 960;

            case 942:
                x[i] ^= 1227;

            case 943:
                x[i] ^= 1948;

            case 944:
                x[i] ^= 279;

            case 945:
                x[i] ^= 3545;

            case 946:
                x[i] ^= 1752;

            case 947:
                x[i] ^= 2784;

            case 948:
                x[i] ^= 2014;

            case 949:
                x[i] ^= 2535;

            case 950:
                x[i] ^= 865;

            case 951:
                x[i] ^= 2879;

            case 952:
                x[i] ^= 3593;

            case 953:
                x[i] ^= 1222;

            case 954:
                x[i] ^= 3009;

            case 955:
                x[i] ^= 3155;

            case 956:
                x[i] ^= 3216;

            case 957:
                x[i] ^= 1929;

            case 958:
                x[i] ^= 1018;

            case 959:
                x[i] ^= 333;

            case 960:
                x[i] ^= 1063;

            case 961:
                x[i] ^= 1451;

            case 962:
                x[i] ^= 1923;

            case 963:
                x[i] ^= 1521;

            case 964:
                x[i] ^= 978;

            case 965:
                x[i] ^= 3983;

            case 966:
                x[i] ^= 3513;

            case 967:
                x[i] ^= 645;

            case 968:
                x[i] ^= 3937;

            case 969:
                x[i] ^= 2499;

            case 970:
                x[i] ^= 2934;

            case 971:
                x[i] ^= 3190;

            case 972:
                x[i] ^= 1747;

            case 973:
                x[i] ^= 2801;

            case 974:
                x[i] ^= 3741;

            case 975:
                x[i] ^= 1318;

            case 976:
                x[i] ^= 4052;

            case 977:
                x[i] ^= 2995;

            case 978:
                x[i] ^= 557;

            case 979:
                x[i] ^= 3859;

            case 980:
                x[i] ^= 1928;

            case 981:
                x[i] ^= 879;

            case 982:
                x[i] ^= 3389;

            case 983:
                x[i] ^= 3224;

            case 984:
                x[i] ^= 546;

            case 985:
                x[i] ^= 2503;

            case 986:
                x[i] ^= 1107;

            case 987:
                x[i] ^= 2506;

            case 988:
                x[i] ^= 770;

            case 989:
                x[i] ^= 3401;

            case 990:
                x[i] ^= 577;

            case 991:
                x[i] ^= 220;

            case 992:
                x[i] ^= 3533;

            case 993:
                x[i] ^= 2339;

            case 994:
                x[i] ^= 2769;

            case 995:
                x[i] ^= 499;

            case 996:
                x[i] ^= 187;

            case 997:
                x[i] ^= 1088;

            case 998:
                x[i] ^= 1927;

            case 999:
                x[i] ^= 3365;

            case 1000:
                x[i] ^= 421;

            case 1001:
                x[i] ^= 3147;

            case 1002:
                x[i] ^= 1421;

            case 1003:
                x[i] ^= 2738;

            case 1004:
                x[i] ^= 3217;

            case 1005:
                x[i] ^= 1350;

            case 1006:
                x[i] ^= 1893;

            case 1007:
                x[i] ^= 2010;

            case 1008:
                x[i] ^= 3170;

            case 1009:
                x[i] ^= 63;

            case 1010:
                x[i] ^= 2593;

            case 1011:
                x[i] ^= 2263;

            case 1012:
                x[i] ^= 2005;

            case 1013:
                x[i] ^= 1497;

            case 1014:
                x[i] ^= 1960;

            case 1015:
                x[i] ^= 2232;

            case 1016:
                x[i] ^= 657;

            case 1017:
                x[i] ^= 817;

            case 1018:
                x[i] ^= 2766;

            case 1019:
                x[i] ^= 597;

            case 1020:
                x[i] ^= 731;

            case 1021:
                x[i] ^= 878;

            case 1022:
                x[i] ^= 1232;

            case 1023:
                x[i] ^= 3307;

            default:
                x[i]++;
                x[i] &= 1023;
            }
        }
    }

    BLACK_BOX(x);

    free(x);
}
