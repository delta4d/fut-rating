FutRating
=========

Calculate rating for a given FUT squad.


Usage
-----

```
$ fut-rating -h
fut_rating 0.1
Calculate FIFA Fut Squad Rating

USAGE:
    fut-rating [OPTIONS] --candidates <RATINGS>... --target <RATING> --with <RATINGS>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --candidates <RATINGS>...    ratings to try
    -l, --record-limits <INT>        records limit [default: 10]
    -t, --target <RATING>            target rating
    -w, --with <RATINGS>...          ratings already have

$ fut-rating -w 88 87 87 86 85 85 -c 85 84 83 -t 86
83      84      85
------------------------
2       0       3
1       2       2
1       1       3
1       0       4
0       4       1
0       3       2
0       2       3
0       1       4
0       0       5
```


Squad Rating Algorithm
----------------------

```
1. SUM = sum of ratings
2. AVG = average of ratings
3. ABOVE_SUM = sum[r - AVG for r in ratings]
4. TOTAL = round(SUM + ABOVE_SUM)
5. RATING = floor(TOTAL / 11)
```