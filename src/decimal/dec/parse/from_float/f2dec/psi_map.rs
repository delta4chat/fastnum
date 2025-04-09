use crate::{
    decimal::{
        dec::{transmute::transmute, ControlBlock},
        Decimal, Sign, Context,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

const PSI: [u64; 64] = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    2251799813685248,
    4503599627370496,
    9007199254740992,
    18014398509481984,
    36028797018963968,
    72057594037927936,
    144115188075855872,
    288230376151711744,
    576460752303423488,
    1152921504606846976,
    2305843009213693952,
    4611686018427387904,
    9223372036854775808,
];

const D_EXP: [i16; 1075] = [
    1,   // b_exp = 0
    1,   // b_exp = 1
    1,   // b_exp = 2
    1,   // b_exp = 3
    2,   // b_exp = 4
    2,   // b_exp = 5
    2,   // b_exp = 6
    3,   // b_exp = 7
    3,   // b_exp = 8
    3,   // b_exp = 9
    4,   // b_exp = 10
    4,   // b_exp = 11
    4,   // b_exp = 12
    4,   // b_exp = 13
    5,   // b_exp = 14
    5,   // b_exp = 15
    5,   // b_exp = 16
    6,   // b_exp = 17
    6,   // b_exp = 18
    6,   // b_exp = 19
    7,   // b_exp = 20
    7,   // b_exp = 21
    7,   // b_exp = 22
    7,   // b_exp = 23
    8,   // b_exp = 24
    8,   // b_exp = 25
    8,   // b_exp = 26
    9,   // b_exp = 27
    9,   // b_exp = 28
    9,   // b_exp = 29
    10,  // b_exp = 30
    10,  // b_exp = 31
    10,  // b_exp = 32
    10,  // b_exp = 33
    11,  // b_exp = 34
    11,  // b_exp = 35
    11,  // b_exp = 36
    12,  // b_exp = 37
    12,  // b_exp = 38
    12,  // b_exp = 39
    13,  // b_exp = 40
    13,  // b_exp = 41
    13,  // b_exp = 42
    13,  // b_exp = 43
    14,  // b_exp = 44
    14,  // b_exp = 45
    14,  // b_exp = 46
    15,  // b_exp = 47
    15,  // b_exp = 48
    15,  // b_exp = 49
    16,  // b_exp = 50
    16,  // b_exp = 51
    16,  // b_exp = 52
    16,  // b_exp = 53
    17,  // b_exp = 54
    17,  // b_exp = 55
    17,  // b_exp = 56
    18,  // b_exp = 57
    18,  // b_exp = 58
    18,  // b_exp = 59
    19,  // b_exp = 60
    19,  // b_exp = 61
    19,  // b_exp = 62
    19,  // b_exp = 63
    20,  // b_exp = 64
    20,  // b_exp = 65
    20,  // b_exp = 66
    21,  // b_exp = 67
    21,  // b_exp = 68
    21,  // b_exp = 69
    22,  // b_exp = 70
    22,  // b_exp = 71
    22,  // b_exp = 72
    22,  // b_exp = 73
    23,  // b_exp = 74
    23,  // b_exp = 75
    23,  // b_exp = 76
    24,  // b_exp = 77
    24,  // b_exp = 78
    24,  // b_exp = 79
    25,  // b_exp = 80
    25,  // b_exp = 81
    25,  // b_exp = 82
    25,  // b_exp = 83
    26,  // b_exp = 84
    26,  // b_exp = 85
    26,  // b_exp = 86
    27,  // b_exp = 87
    27,  // b_exp = 88
    27,  // b_exp = 89
    28,  // b_exp = 90
    28,  // b_exp = 91
    28,  // b_exp = 92
    28,  // b_exp = 93
    29,  // b_exp = 94
    29,  // b_exp = 95
    29,  // b_exp = 96
    30,  // b_exp = 97
    30,  // b_exp = 98
    30,  // b_exp = 99
    31,  // b_exp = 100
    31,  // b_exp = 101
    31,  // b_exp = 102
    32,  // b_exp = 103
    32,  // b_exp = 104
    32,  // b_exp = 105
    32,  // b_exp = 106
    33,  // b_exp = 107
    33,  // b_exp = 108
    33,  // b_exp = 109
    34,  // b_exp = 110
    34,  // b_exp = 111
    34,  // b_exp = 112
    35,  // b_exp = 113
    35,  // b_exp = 114
    35,  // b_exp = 115
    35,  // b_exp = 116
    36,  // b_exp = 117
    36,  // b_exp = 118
    36,  // b_exp = 119
    37,  // b_exp = 120
    37,  // b_exp = 121
    37,  // b_exp = 122
    38,  // b_exp = 123
    38,  // b_exp = 124
    38,  // b_exp = 125
    38,  // b_exp = 126
    39,  // b_exp = 127
    39,  // b_exp = 128
    39,  // b_exp = 129
    40,  // b_exp = 130
    40,  // b_exp = 131
    40,  // b_exp = 132
    41,  // b_exp = 133
    41,  // b_exp = 134
    41,  // b_exp = 135
    41,  // b_exp = 136
    42,  // b_exp = 137
    42,  // b_exp = 138
    42,  // b_exp = 139
    43,  // b_exp = 140
    43,  // b_exp = 141
    43,  // b_exp = 142
    44,  // b_exp = 143
    44,  // b_exp = 144
    44,  // b_exp = 145
    44,  // b_exp = 146
    45,  // b_exp = 147
    45,  // b_exp = 148
    45,  // b_exp = 149
    46,  // b_exp = 150
    46,  // b_exp = 151
    46,  // b_exp = 152
    47,  // b_exp = 153
    47,  // b_exp = 154
    47,  // b_exp = 155
    47,  // b_exp = 156
    48,  // b_exp = 157
    48,  // b_exp = 158
    48,  // b_exp = 159
    49,  // b_exp = 160
    49,  // b_exp = 161
    49,  // b_exp = 162
    50,  // b_exp = 163
    50,  // b_exp = 164
    50,  // b_exp = 165
    50,  // b_exp = 166
    51,  // b_exp = 167
    51,  // b_exp = 168
    51,  // b_exp = 169
    52,  // b_exp = 170
    52,  // b_exp = 171
    52,  // b_exp = 172
    53,  // b_exp = 173
    53,  // b_exp = 174
    53,  // b_exp = 175
    53,  // b_exp = 176
    54,  // b_exp = 177
    54,  // b_exp = 178
    54,  // b_exp = 179
    55,  // b_exp = 180
    55,  // b_exp = 181
    55,  // b_exp = 182
    56,  // b_exp = 183
    56,  // b_exp = 184
    56,  // b_exp = 185
    56,  // b_exp = 186
    57,  // b_exp = 187
    57,  // b_exp = 188
    57,  // b_exp = 189
    58,  // b_exp = 190
    58,  // b_exp = 191
    58,  // b_exp = 192
    59,  // b_exp = 193
    59,  // b_exp = 194
    59,  // b_exp = 195
    60,  // b_exp = 196
    60,  // b_exp = 197
    60,  // b_exp = 198
    60,  // b_exp = 199
    61,  // b_exp = 200
    61,  // b_exp = 201
    61,  // b_exp = 202
    62,  // b_exp = 203
    62,  // b_exp = 204
    62,  // b_exp = 205
    63,  // b_exp = 206
    63,  // b_exp = 207
    63,  // b_exp = 208
    63,  // b_exp = 209
    64,  // b_exp = 210
    64,  // b_exp = 211
    64,  // b_exp = 212
    65,  // b_exp = 213
    65,  // b_exp = 214
    65,  // b_exp = 215
    66,  // b_exp = 216
    66,  // b_exp = 217
    66,  // b_exp = 218
    66,  // b_exp = 219
    67,  // b_exp = 220
    67,  // b_exp = 221
    67,  // b_exp = 222
    68,  // b_exp = 223
    68,  // b_exp = 224
    68,  // b_exp = 225
    69,  // b_exp = 226
    69,  // b_exp = 227
    69,  // b_exp = 228
    69,  // b_exp = 229
    70,  // b_exp = 230
    70,  // b_exp = 231
    70,  // b_exp = 232
    71,  // b_exp = 233
    71,  // b_exp = 234
    71,  // b_exp = 235
    72,  // b_exp = 236
    72,  // b_exp = 237
    72,  // b_exp = 238
    72,  // b_exp = 239
    73,  // b_exp = 240
    73,  // b_exp = 241
    73,  // b_exp = 242
    74,  // b_exp = 243
    74,  // b_exp = 244
    74,  // b_exp = 245
    75,  // b_exp = 246
    75,  // b_exp = 247
    75,  // b_exp = 248
    75,  // b_exp = 249
    76,  // b_exp = 250
    76,  // b_exp = 251
    76,  // b_exp = 252
    77,  // b_exp = 253
    77,  // b_exp = 254
    77,  // b_exp = 255
    78,  // b_exp = 256
    78,  // b_exp = 257
    78,  // b_exp = 258
    78,  // b_exp = 259
    79,  // b_exp = 260
    79,  // b_exp = 261
    79,  // b_exp = 262
    80,  // b_exp = 263
    80,  // b_exp = 264
    80,  // b_exp = 265
    81,  // b_exp = 266
    81,  // b_exp = 267
    81,  // b_exp = 268
    81,  // b_exp = 269
    82,  // b_exp = 270
    82,  // b_exp = 271
    82,  // b_exp = 272
    83,  // b_exp = 273
    83,  // b_exp = 274
    83,  // b_exp = 275
    84,  // b_exp = 276
    84,  // b_exp = 277
    84,  // b_exp = 278
    84,  // b_exp = 279
    85,  // b_exp = 280
    85,  // b_exp = 281
    85,  // b_exp = 282
    86,  // b_exp = 283
    86,  // b_exp = 284
    86,  // b_exp = 285
    87,  // b_exp = 286
    87,  // b_exp = 287
    87,  // b_exp = 288
    87,  // b_exp = 289
    88,  // b_exp = 290
    88,  // b_exp = 291
    88,  // b_exp = 292
    89,  // b_exp = 293
    89,  // b_exp = 294
    89,  // b_exp = 295
    90,  // b_exp = 296
    90,  // b_exp = 297
    90,  // b_exp = 298
    91,  // b_exp = 299
    91,  // b_exp = 300
    91,  // b_exp = 301
    91,  // b_exp = 302
    92,  // b_exp = 303
    92,  // b_exp = 304
    92,  // b_exp = 305
    93,  // b_exp = 306
    93,  // b_exp = 307
    93,  // b_exp = 308
    94,  // b_exp = 309
    94,  // b_exp = 310
    94,  // b_exp = 311
    94,  // b_exp = 312
    95,  // b_exp = 313
    95,  // b_exp = 314
    95,  // b_exp = 315
    96,  // b_exp = 316
    96,  // b_exp = 317
    96,  // b_exp = 318
    97,  // b_exp = 319
    97,  // b_exp = 320
    97,  // b_exp = 321
    97,  // b_exp = 322
    98,  // b_exp = 323
    98,  // b_exp = 324
    98,  // b_exp = 325
    99,  // b_exp = 326
    99,  // b_exp = 327
    99,  // b_exp = 328
    100, // b_exp = 329
    100, // b_exp = 330
    100, // b_exp = 331
    100, // b_exp = 332
    101, // b_exp = 333
    101, // b_exp = 334
    101, // b_exp = 335
    102, // b_exp = 336
    102, // b_exp = 337
    102, // b_exp = 338
    103, // b_exp = 339
    103, // b_exp = 340
    103, // b_exp = 341
    103, // b_exp = 342
    104, // b_exp = 343
    104, // b_exp = 344
    104, // b_exp = 345
    105, // b_exp = 346
    105, // b_exp = 347
    105, // b_exp = 348
    106, // b_exp = 349
    106, // b_exp = 350
    106, // b_exp = 351
    106, // b_exp = 352
    107, // b_exp = 353
    107, // b_exp = 354
    107, // b_exp = 355
    108, // b_exp = 356
    108, // b_exp = 357
    108, // b_exp = 358
    109, // b_exp = 359
    109, // b_exp = 360
    109, // b_exp = 361
    109, // b_exp = 362
    110, // b_exp = 363
    110, // b_exp = 364
    110, // b_exp = 365
    111, // b_exp = 366
    111, // b_exp = 367
    111, // b_exp = 368
    112, // b_exp = 369
    112, // b_exp = 370
    112, // b_exp = 371
    112, // b_exp = 372
    113, // b_exp = 373
    113, // b_exp = 374
    113, // b_exp = 375
    114, // b_exp = 376
    114, // b_exp = 377
    114, // b_exp = 378
    115, // b_exp = 379
    115, // b_exp = 380
    115, // b_exp = 381
    115, // b_exp = 382
    116, // b_exp = 383
    116, // b_exp = 384
    116, // b_exp = 385
    117, // b_exp = 386
    117, // b_exp = 387
    117, // b_exp = 388
    118, // b_exp = 389
    118, // b_exp = 390
    118, // b_exp = 391
    119, // b_exp = 392
    119, // b_exp = 393
    119, // b_exp = 394
    119, // b_exp = 395
    120, // b_exp = 396
    120, // b_exp = 397
    120, // b_exp = 398
    121, // b_exp = 399
    121, // b_exp = 400
    121, // b_exp = 401
    122, // b_exp = 402
    122, // b_exp = 403
    122, // b_exp = 404
    122, // b_exp = 405
    123, // b_exp = 406
    123, // b_exp = 407
    123, // b_exp = 408
    124, // b_exp = 409
    124, // b_exp = 410
    124, // b_exp = 411
    125, // b_exp = 412
    125, // b_exp = 413
    125, // b_exp = 414
    125, // b_exp = 415
    126, // b_exp = 416
    126, // b_exp = 417
    126, // b_exp = 418
    127, // b_exp = 419
    127, // b_exp = 420
    127, // b_exp = 421
    128, // b_exp = 422
    128, // b_exp = 423
    128, // b_exp = 424
    128, // b_exp = 425
    129, // b_exp = 426
    129, // b_exp = 427
    129, // b_exp = 428
    130, // b_exp = 429
    130, // b_exp = 430
    130, // b_exp = 431
    131, // b_exp = 432
    131, // b_exp = 433
    131, // b_exp = 434
    131, // b_exp = 435
    132, // b_exp = 436
    132, // b_exp = 437
    132, // b_exp = 438
    133, // b_exp = 439
    133, // b_exp = 440
    133, // b_exp = 441
    134, // b_exp = 442
    134, // b_exp = 443
    134, // b_exp = 444
    134, // b_exp = 445
    135, // b_exp = 446
    135, // b_exp = 447
    135, // b_exp = 448
    136, // b_exp = 449
    136, // b_exp = 450
    136, // b_exp = 451
    137, // b_exp = 452
    137, // b_exp = 453
    137, // b_exp = 454
    137, // b_exp = 455
    138, // b_exp = 456
    138, // b_exp = 457
    138, // b_exp = 458
    139, // b_exp = 459
    139, // b_exp = 460
    139, // b_exp = 461
    140, // b_exp = 462
    140, // b_exp = 463
    140, // b_exp = 464
    140, // b_exp = 465
    141, // b_exp = 466
    141, // b_exp = 467
    141, // b_exp = 468
    142, // b_exp = 469
    142, // b_exp = 470
    142, // b_exp = 471
    143, // b_exp = 472
    143, // b_exp = 473
    143, // b_exp = 474
    143, // b_exp = 475
    144, // b_exp = 476
    144, // b_exp = 477
    144, // b_exp = 478
    145, // b_exp = 479
    145, // b_exp = 480
    145, // b_exp = 481
    146, // b_exp = 482
    146, // b_exp = 483
    146, // b_exp = 484
    146, // b_exp = 485
    147, // b_exp = 486
    147, // b_exp = 487
    147, // b_exp = 488
    148, // b_exp = 489
    148, // b_exp = 490
    148, // b_exp = 491
    149, // b_exp = 492
    149, // b_exp = 493
    149, // b_exp = 494
    150, // b_exp = 495
    150, // b_exp = 496
    150, // b_exp = 497
    150, // b_exp = 498
    151, // b_exp = 499
    151, // b_exp = 500
    151, // b_exp = 501
    152, // b_exp = 502
    152, // b_exp = 503
    152, // b_exp = 504
    153, // b_exp = 505
    153, // b_exp = 506
    153, // b_exp = 507
    153, // b_exp = 508
    154, // b_exp = 509
    154, // b_exp = 510
    154, // b_exp = 511
    155, // b_exp = 512
    155, // b_exp = 513
    155, // b_exp = 514
    156, // b_exp = 515
    156, // b_exp = 516
    156, // b_exp = 517
    156, // b_exp = 518
    157, // b_exp = 519
    157, // b_exp = 520
    157, // b_exp = 521
    158, // b_exp = 522
    158, // b_exp = 523
    158, // b_exp = 524
    159, // b_exp = 525
    159, // b_exp = 526
    159, // b_exp = 527
    159, // b_exp = 528
    160, // b_exp = 529
    160, // b_exp = 530
    160, // b_exp = 531
    161, // b_exp = 532
    161, // b_exp = 533
    161, // b_exp = 534
    162, // b_exp = 535
    162, // b_exp = 536
    162, // b_exp = 537
    162, // b_exp = 538
    163, // b_exp = 539
    163, // b_exp = 540
    163, // b_exp = 541
    164, // b_exp = 542
    164, // b_exp = 543
    164, // b_exp = 544
    165, // b_exp = 545
    165, // b_exp = 546
    165, // b_exp = 547
    165, // b_exp = 548
    166, // b_exp = 549
    166, // b_exp = 550
    166, // b_exp = 551
    167, // b_exp = 552
    167, // b_exp = 553
    167, // b_exp = 554
    168, // b_exp = 555
    168, // b_exp = 556
    168, // b_exp = 557
    168, // b_exp = 558
    169, // b_exp = 559
    169, // b_exp = 560
    169, // b_exp = 561
    170, // b_exp = 562
    170, // b_exp = 563
    170, // b_exp = 564
    171, // b_exp = 565
    171, // b_exp = 566
    171, // b_exp = 567
    171, // b_exp = 568
    172, // b_exp = 569
    172, // b_exp = 570
    172, // b_exp = 571
    173, // b_exp = 572
    173, // b_exp = 573
    173, // b_exp = 574
    174, // b_exp = 575
    174, // b_exp = 576
    174, // b_exp = 577
    174, // b_exp = 578
    175, // b_exp = 579
    175, // b_exp = 580
    175, // b_exp = 581
    176, // b_exp = 582
    176, // b_exp = 583
    176, // b_exp = 584
    177, // b_exp = 585
    177, // b_exp = 586
    177, // b_exp = 587
    178, // b_exp = 588
    178, // b_exp = 589
    178, // b_exp = 590
    178, // b_exp = 591
    179, // b_exp = 592
    179, // b_exp = 593
    179, // b_exp = 594
    180, // b_exp = 595
    180, // b_exp = 596
    180, // b_exp = 597
    181, // b_exp = 598
    181, // b_exp = 599
    181, // b_exp = 600
    181, // b_exp = 601
    182, // b_exp = 602
    182, // b_exp = 603
    182, // b_exp = 604
    183, // b_exp = 605
    183, // b_exp = 606
    183, // b_exp = 607
    184, // b_exp = 608
    184, // b_exp = 609
    184, // b_exp = 610
    184, // b_exp = 611
    185, // b_exp = 612
    185, // b_exp = 613
    185, // b_exp = 614
    186, // b_exp = 615
    186, // b_exp = 616
    186, // b_exp = 617
    187, // b_exp = 618
    187, // b_exp = 619
    187, // b_exp = 620
    187, // b_exp = 621
    188, // b_exp = 622
    188, // b_exp = 623
    188, // b_exp = 624
    189, // b_exp = 625
    189, // b_exp = 626
    189, // b_exp = 627
    190, // b_exp = 628
    190, // b_exp = 629
    190, // b_exp = 630
    190, // b_exp = 631
    191, // b_exp = 632
    191, // b_exp = 633
    191, // b_exp = 634
    192, // b_exp = 635
    192, // b_exp = 636
    192, // b_exp = 637
    193, // b_exp = 638
    193, // b_exp = 639
    193, // b_exp = 640
    193, // b_exp = 641
    194, // b_exp = 642
    194, // b_exp = 643
    194, // b_exp = 644
    195, // b_exp = 645
    195, // b_exp = 646
    195, // b_exp = 647
    196, // b_exp = 648
    196, // b_exp = 649
    196, // b_exp = 650
    196, // b_exp = 651
    197, // b_exp = 652
    197, // b_exp = 653
    197, // b_exp = 654
    198, // b_exp = 655
    198, // b_exp = 656
    198, // b_exp = 657
    199, // b_exp = 658
    199, // b_exp = 659
    199, // b_exp = 660
    199, // b_exp = 661
    200, // b_exp = 662
    200, // b_exp = 663
    200, // b_exp = 664
    201, // b_exp = 665
    201, // b_exp = 666
    201, // b_exp = 667
    202, // b_exp = 668
    202, // b_exp = 669
    202, // b_exp = 670
    202, // b_exp = 671
    203, // b_exp = 672
    203, // b_exp = 673
    203, // b_exp = 674
    204, // b_exp = 675
    204, // b_exp = 676
    204, // b_exp = 677
    205, // b_exp = 678
    205, // b_exp = 679
    205, // b_exp = 680
    206, // b_exp = 681
    206, // b_exp = 682
    206, // b_exp = 683
    206, // b_exp = 684
    207, // b_exp = 685
    207, // b_exp = 686
    207, // b_exp = 687
    208, // b_exp = 688
    208, // b_exp = 689
    208, // b_exp = 690
    209, // b_exp = 691
    209, // b_exp = 692
    209, // b_exp = 693
    209, // b_exp = 694
    210, // b_exp = 695
    210, // b_exp = 696
    210, // b_exp = 697
    211, // b_exp = 698
    211, // b_exp = 699
    211, // b_exp = 700
    212, // b_exp = 701
    212, // b_exp = 702
    212, // b_exp = 703
    212, // b_exp = 704
    213, // b_exp = 705
    213, // b_exp = 706
    213, // b_exp = 707
    214, // b_exp = 708
    214, // b_exp = 709
    214, // b_exp = 710
    215, // b_exp = 711
    215, // b_exp = 712
    215, // b_exp = 713
    215, // b_exp = 714
    216, // b_exp = 715
    216, // b_exp = 716
    216, // b_exp = 717
    217, // b_exp = 718
    217, // b_exp = 719
    217, // b_exp = 720
    218, // b_exp = 721
    218, // b_exp = 722
    218, // b_exp = 723
    218, // b_exp = 724
    219, // b_exp = 725
    219, // b_exp = 726
    219, // b_exp = 727
    220, // b_exp = 728
    220, // b_exp = 729
    220, // b_exp = 730
    221, // b_exp = 731
    221, // b_exp = 732
    221, // b_exp = 733
    221, // b_exp = 734
    222, // b_exp = 735
    222, // b_exp = 736
    222, // b_exp = 737
    223, // b_exp = 738
    223, // b_exp = 739
    223, // b_exp = 740
    224, // b_exp = 741
    224, // b_exp = 742
    224, // b_exp = 743
    224, // b_exp = 744
    225, // b_exp = 745
    225, // b_exp = 746
    225, // b_exp = 747
    226, // b_exp = 748
    226, // b_exp = 749
    226, // b_exp = 750
    227, // b_exp = 751
    227, // b_exp = 752
    227, // b_exp = 753
    227, // b_exp = 754
    228, // b_exp = 755
    228, // b_exp = 756
    228, // b_exp = 757
    229, // b_exp = 758
    229, // b_exp = 759
    229, // b_exp = 760
    230, // b_exp = 761
    230, // b_exp = 762
    230, // b_exp = 763
    230, // b_exp = 764
    231, // b_exp = 765
    231, // b_exp = 766
    231, // b_exp = 767
    232, // b_exp = 768
    232, // b_exp = 769
    232, // b_exp = 770
    233, // b_exp = 771
    233, // b_exp = 772
    233, // b_exp = 773
    233, // b_exp = 774
    234, // b_exp = 775
    234, // b_exp = 776
    234, // b_exp = 777
    235, // b_exp = 778
    235, // b_exp = 779
    235, // b_exp = 780
    236, // b_exp = 781
    236, // b_exp = 782
    236, // b_exp = 783
    237, // b_exp = 784
    237, // b_exp = 785
    237, // b_exp = 786
    237, // b_exp = 787
    238, // b_exp = 788
    238, // b_exp = 789
    238, // b_exp = 790
    239, // b_exp = 791
    239, // b_exp = 792
    239, // b_exp = 793
    240, // b_exp = 794
    240, // b_exp = 795
    240, // b_exp = 796
    240, // b_exp = 797
    241, // b_exp = 798
    241, // b_exp = 799
    241, // b_exp = 800
    242, // b_exp = 801
    242, // b_exp = 802
    242, // b_exp = 803
    243, // b_exp = 804
    243, // b_exp = 805
    243, // b_exp = 806
    243, // b_exp = 807
    244, // b_exp = 808
    244, // b_exp = 809
    244, // b_exp = 810
    245, // b_exp = 811
    245, // b_exp = 812
    245, // b_exp = 813
    246, // b_exp = 814
    246, // b_exp = 815
    246, // b_exp = 816
    246, // b_exp = 817
    247, // b_exp = 818
    247, // b_exp = 819
    247, // b_exp = 820
    248, // b_exp = 821
    248, // b_exp = 822
    248, // b_exp = 823
    249, // b_exp = 824
    249, // b_exp = 825
    249, // b_exp = 826
    249, // b_exp = 827
    250, // b_exp = 828
    250, // b_exp = 829
    250, // b_exp = 830
    251, // b_exp = 831
    251, // b_exp = 832
    251, // b_exp = 833
    252, // b_exp = 834
    252, // b_exp = 835
    252, // b_exp = 836
    252, // b_exp = 837
    253, // b_exp = 838
    253, // b_exp = 839
    253, // b_exp = 840
    254, // b_exp = 841
    254, // b_exp = 842
    254, // b_exp = 843
    255, // b_exp = 844
    255, // b_exp = 845
    255, // b_exp = 846
    255, // b_exp = 847
    256, // b_exp = 848
    256, // b_exp = 849
    256, // b_exp = 850
    257, // b_exp = 851
    257, // b_exp = 852
    257, // b_exp = 853
    258, // b_exp = 854
    258, // b_exp = 855
    258, // b_exp = 856
    258, // b_exp = 857
    259, // b_exp = 858
    259, // b_exp = 859
    259, // b_exp = 860
    260, // b_exp = 861
    260, // b_exp = 862
    260, // b_exp = 863
    261, // b_exp = 864
    261, // b_exp = 865
    261, // b_exp = 866
    261, // b_exp = 867
    262, // b_exp = 868
    262, // b_exp = 869
    262, // b_exp = 870
    263, // b_exp = 871
    263, // b_exp = 872
    263, // b_exp = 873
    264, // b_exp = 874
    264, // b_exp = 875
    264, // b_exp = 876
    265, // b_exp = 877
    265, // b_exp = 878
    265, // b_exp = 879
    265, // b_exp = 880
    266, // b_exp = 881
    266, // b_exp = 882
    266, // b_exp = 883
    267, // b_exp = 884
    267, // b_exp = 885
    267, // b_exp = 886
    268, // b_exp = 887
    268, // b_exp = 888
    268, // b_exp = 889
    268, // b_exp = 890
    269, // b_exp = 891
    269, // b_exp = 892
    269, // b_exp = 893
    270, // b_exp = 894
    270, // b_exp = 895
    270, // b_exp = 896
    271, // b_exp = 897
    271, // b_exp = 898
    271, // b_exp = 899
    271, // b_exp = 900
    272, // b_exp = 901
    272, // b_exp = 902
    272, // b_exp = 903
    273, // b_exp = 904
    273, // b_exp = 905
    273, // b_exp = 906
    274, // b_exp = 907
    274, // b_exp = 908
    274, // b_exp = 909
    274, // b_exp = 910
    275, // b_exp = 911
    275, // b_exp = 912
    275, // b_exp = 913
    276, // b_exp = 914
    276, // b_exp = 915
    276, // b_exp = 916
    277, // b_exp = 917
    277, // b_exp = 918
    277, // b_exp = 919
    277, // b_exp = 920
    278, // b_exp = 921
    278, // b_exp = 922
    278, // b_exp = 923
    279, // b_exp = 924
    279, // b_exp = 925
    279, // b_exp = 926
    280, // b_exp = 927
    280, // b_exp = 928
    280, // b_exp = 929
    280, // b_exp = 930
    281, // b_exp = 931
    281, // b_exp = 932
    281, // b_exp = 933
    282, // b_exp = 934
    282, // b_exp = 935
    282, // b_exp = 936
    283, // b_exp = 937
    283, // b_exp = 938
    283, // b_exp = 939
    283, // b_exp = 940
    284, // b_exp = 941
    284, // b_exp = 942
    284, // b_exp = 943
    285, // b_exp = 944
    285, // b_exp = 945
    285, // b_exp = 946
    286, // b_exp = 947
    286, // b_exp = 948
    286, // b_exp = 949
    286, // b_exp = 950
    287, // b_exp = 951
    287, // b_exp = 952
    287, // b_exp = 953
    288, // b_exp = 954
    288, // b_exp = 955
    288, // b_exp = 956
    289, // b_exp = 957
    289, // b_exp = 958
    289, // b_exp = 959
    289, // b_exp = 960
    290, // b_exp = 961
    290, // b_exp = 962
    290, // b_exp = 963
    291, // b_exp = 964
    291, // b_exp = 965
    291, // b_exp = 966
    292, // b_exp = 967
    292, // b_exp = 968
    292, // b_exp = 969
    292, // b_exp = 970
    293, // b_exp = 971
    293, // b_exp = 972
    293, // b_exp = 973
    294, // b_exp = 974
    294, // b_exp = 975
    294, // b_exp = 976
    295, // b_exp = 977
    295, // b_exp = 978
    295, // b_exp = 979
    296, // b_exp = 980
    296, // b_exp = 981
    296, // b_exp = 982
    296, // b_exp = 983
    297, // b_exp = 984
    297, // b_exp = 985
    297, // b_exp = 986
    298, // b_exp = 987
    298, // b_exp = 988
    298, // b_exp = 989
    299, // b_exp = 990
    299, // b_exp = 991
    299, // b_exp = 992
    299, // b_exp = 993
    300, // b_exp = 994
    300, // b_exp = 995
    300, // b_exp = 996
    301, // b_exp = 997
    301, // b_exp = 998
    301, // b_exp = 999
    302, // b_exp = 1000
    302, // b_exp = 1001
    302, // b_exp = 1002
    302, // b_exp = 1003
    303, // b_exp = 1004
    303, // b_exp = 1005
    303, // b_exp = 1006
    304, // b_exp = 1007
    304, // b_exp = 1008
    304, // b_exp = 1009
    305, // b_exp = 1010
    305, // b_exp = 1011
    305, // b_exp = 1012
    305, // b_exp = 1013
    306, // b_exp = 1014
    306, // b_exp = 1015
    306, // b_exp = 1016
    307, // b_exp = 1017
    307, // b_exp = 1018
    307, // b_exp = 1019
    308, // b_exp = 1020
    308, // b_exp = 1021
    308, // b_exp = 1022
    308, // b_exp = 1023
    309, // b_exp = 1024
    309, // b_exp = 1025
    309, // b_exp = 1026
    310, // b_exp = 1027
    310, // b_exp = 1028
    310, // b_exp = 1029
    311, // b_exp = 1030
    311, // b_exp = 1031
    311, // b_exp = 1032
    311, // b_exp = 1033
    312, // b_exp = 1034
    312, // b_exp = 1035
    312, // b_exp = 1036
    313, // b_exp = 1037
    313, // b_exp = 1038
    313, // b_exp = 1039
    314, // b_exp = 1040
    314, // b_exp = 1041
    314, // b_exp = 1042
    314, // b_exp = 1043
    315, // b_exp = 1044
    315, // b_exp = 1045
    315, // b_exp = 1046
    316, // b_exp = 1047
    316, // b_exp = 1048
    316, // b_exp = 1049
    317, // b_exp = 1050
    317, // b_exp = 1051
    317, // b_exp = 1052
    317, // b_exp = 1053
    318, // b_exp = 1054
    318, // b_exp = 1055
    318, // b_exp = 1056
    319, // b_exp = 1057
    319, // b_exp = 1058
    319, // b_exp = 1059
    320, // b_exp = 1060
    320, // b_exp = 1061
    320, // b_exp = 1062
    320, // b_exp = 1063
    321, // b_exp = 1064
    321, // b_exp = 1065
    321, // b_exp = 1066
    322, // b_exp = 1067
    322, // b_exp = 1068
    322, // b_exp = 1069
    323, // b_exp = 1070
    323, // b_exp = 1071
    323, // b_exp = 1072
    324, // b_exp = 1073
    324, // b_exp = 1074
];

#[inline]
pub(crate) const fn lookup<const N: usize>(b_exp: i16) -> (i16, D<N>) {
    debug_assert!(b_exp >= 0 && b_exp <= 1074);

    let d_exp = D_EXP[b_exp as usize];

    let psi = if b_exp < 64 {
        transmute(D::<1>::new(
            UInt::from_digits([PSI[b_exp as usize]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 128 {
        transmute(D::<2>::new(
            UInt::from_digits([0, PSI[b_exp as usize - 64]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 192 {
        transmute(D::<3>::new(
            UInt::from_digits([0, 0, PSI[b_exp as usize - 128]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 256 {
        transmute(D::<4>::new(
            UInt::from_digits([0, 0, 0, PSI[b_exp as usize - 192]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 320 {
        transmute(D::<5>::new(
            UInt::from_digits([0, 0, 0, 0, PSI[b_exp as usize - 256]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 384 {
        transmute(D::<6>::new(
            UInt::from_digits([0, 0, 0, 0, 0, PSI[b_exp as usize - 320]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 448 {
        transmute(D::<7>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 384]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 512 {
        transmute(D::<8>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 448]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 576 {
        transmute(D::<9>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 512]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 640 {
        transmute(D::<10>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 576]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 704 {
        transmute(D::<11>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 640]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 768 {
        transmute(D::<12>::new(
            UInt::from_digits([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, PSI[b_exp as usize - 704]]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 832 {
        transmute(D::<13>::new(
            UInt::from_digits([
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PSI[b_exp as usize - 768],
            ]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 896 {
        transmute(D::<14>::new(
            UInt::from_digits([
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PSI[b_exp as usize - 832],
            ]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 960 {
        transmute(D::<15>::new(
            UInt::from_digits([
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PSI[b_exp as usize - 896],
            ]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else if b_exp < 1024 {
        transmute(D::<16>::new(
            UInt::from_digits([
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PSI[b_exp as usize - 960],
            ]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    } else {
        transmute(D::<17>::new(
            UInt::from_digits([
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PSI[b_exp as usize - 1024],
            ]),
            ControlBlock::basic(d_exp, Sign::Plus),
            Context::DEFAULT,
        ))
    };

    (d_exp, psi)
}
