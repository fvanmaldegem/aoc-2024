MODULE helper
    IMPLICIT NONE
    
    PUBLIC append_int
contains

    SUBROUTINE append_int(array, x)
        INTEGER, ALLOCATABLE, DIMENSION(:), INTENT(INOUT) :: array
        INTEGER, intent(IN) :: x

        INTEGER, ALLOCATABLE, DIMENSION(:) :: tmp
        INTEGER :: s

        s = SIZE(array)

        ALLOCATE( tmp(s) )
        CALL move_alloc(array, tmp)

        ALLOCATE( array(s+1) )

        array(:s)  = tmp
        array(s+1) = x

        DEALLOCATE(tmp)

    END SUBROUTINE append_int
    
end MODULE helper