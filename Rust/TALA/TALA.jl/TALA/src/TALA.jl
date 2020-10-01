module TALA


    function __init__()
        try
            using PyCall
        catch err
            println("PyCall MUST be installed prior to installing TALA")
        end

        try
            TALA = pyimport("TALA")
        catch err
            println("Please enter location of TALA wheel file: ")
            pip = pyimport("pip")
            pip.main(["install", readline()])
            TALA = pyimport("TALA")
        end

    end



end # module
