PROGRAM solution_01
    USE stdlib_io, ONLY: open, getline, FMT_INT
    USE stdlib_str2num, ONLY: to_num
    USE M_strings, ONLY: split
    USE helper, ONLY: append_int

    IMPLICIT NONE

    INTEGER :: u, stat, x, i
    CHARACTER(len=:), ALLOCATABLE :: line
    CHARACTER(len=:), ALLOCATABLE, DIMENSION(:) :: strSplit
    INTEGER, ALLOCATABLE, DIMENSION(:) :: left, right

    ALLOCATE(left(0))
    ALLOCATE(right(0))

    u = open('inputs/01.txt')
    CALL getline(u, line, stat)
    i = 0
    DO WHILE(stat == 0)
        ! Increment counter
        i = i + 1

        ! Split the line and store in strSplit
        CALL split(line, strSplit, '   ')

        ! Append to left
        x  = to_num(strSplit(1), x)
        CALL append_int(left, x)

        ! Append to right
        x  = to_num(strSplit(2), x)
        CALL append_int(right, x)
        
        ! Get the next line
        CALL getline(u, line, stat)
    END DO
    close(u)

    CALL solve1(left, right)
    CALL solve2(left, right)
CONTAINS
    SUBROUTINE solve1(l, r)
        IMPLICIT NONE
        INTEGER, ALLOCATABLE, DIMENSION(:), INTENT(IN) :: l, r

        INTEGER, DIMENSION(size(l)) :: tmp_l, tmp_r
        INTEGER :: n, distance, diff, loc, largest_l, largest_r

        distance = 0
        diff     = 0
        tmp_l    = l
        tmp_r    = r

        DO n=1, size(tmp_l)
            loc       = maxloc(tmp_l, 1)
            largest_l = tmp_l(loc)
            tmp_l(loc)    = 0

            loc       = maxloc(tmp_r, 1)
            largest_r = tmp_r(loc)
            tmp_r(loc)    = 0
    
            diff = abs(largest_l - largest_r)
            distance = distance + diff
        END DO

        PRINT '(a)', "Answer to problem 1: "
        PRINT FMT_INT, distance
    END SUBROUTINE

    SUBROUTINE solve2(l, r)
        IMPLICIT NONE
        INTEGER, ALLOCATABLE, DIMENSION(:), INTENT(IN) :: l, r

        INTEGER :: n, k, similarity_score, occurences, current_number

        similarity_score = 0

        DO n=1, size(l)
            current_number = l(n)

            occurences = 0
            DO k=1, size(r)
                IF (r(k) == current_number) THEN
                    occurences = occurences + 1
                END IF
            END DO

            similarity_score = similarity_score + (occurences * current_number)
        END DO

        PRINT '(a)', "Answer to problem 2: "
        PRINT FMT_INT, similarity_score
    END SUBROUTINE
END PROGRAM solution_01