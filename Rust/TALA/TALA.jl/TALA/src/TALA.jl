module TALA

using PyCall

function __init__()

    try
        py"""
        import TALA
        """
    catch
        pip = pyimport("pip")
        pip.main(["install", "TALA"])
    end

end


function host()
    py"""
    TALA.host()
    """
end

function installTest()
    py"""
    TALA.installTest()
    """
end



end # module
