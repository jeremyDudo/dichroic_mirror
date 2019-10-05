using LinearAlgebra, Plots

#-------------------------------------------------------------------------------------------
# FUNCTIONS 

# Fresnel Coefficients for both TE and TM polarized light
function De(n, theta)
    return [ 1 1 ; n*cos(theta) -n*cos(theta)]
end
function Dm(n, theta)
    return [cos(theta) -cos(theta) ; n n ]
end

# determine k for each wavelength of visible light, incident on the surface at angle theta
function kFunc(n, λ, theta)
    """
    n: between ~1-10
    lambda: in nanometers (400-700)
    theta: angle for particular medium (in degrees)
    """
    # c0 = 2.998e+17;                     # nm/s
    return (n*2*pi)/(λ) * cos(theta)
end

# Propagation matrix
function P(n,d,λ,theta)
    k = kFunc(n,λ,theta)
    return [ exp(k*d*im) 0 ; 0 exp(-k*d*im)]
end

# stack the mirror
function mirror_stack(n1,n2,d1,d2,dSub,totalLayers,Polarization,λ)

    # Snell's law to determine angles in a repeating media
    theta1 = asin(sin(45)/n1);
    theta2 = asin(n1*sin(theta1)/n2);

    # Check polarization 
    if Polarization == "TE"
        D = De
    elseif Polarization == "TM"
        D = Dm
    else 
        println("Polarization must be either \"TE\" or \"TM\"")
    end

    # be very careful with number of layers here, can break
    # case 1 should have an even number of layers
    # case 2 should have an odd number of layers
    if totalLayers//2 == 0
        nSub = n1
        thetaSub = theta1         
        return inv(D(nSub, dSub))*D(1, 45) * P(nSub, dSub, λ, thetaSub) * ( inv(D(n1,d1)) * D(n2,d2) * P(n2, d2, λ, theta2) * inv(D(n2,d2)) * D(n1,d1) * P(n1, d1, λ, theta1) )^((totalLayers)/2) * inv(D(1,45))*D(n1,theta1)
    else
        nSub = n2
        thetaSub = theta2 # inv(D(nSub, dSub))*D(1, 45) *
        return inv(D(nSub, dSub))*D(1, 45) * P(nSub, dSub, λ, thetaSub) * inv(D(n2,d2)) * D(n1,d1) * P(n1, d1, λ, theta1) * ( inv(D(n1,d1)) * D(n2,d2) * P(n2, d2, λ, theta2) * inv(D(n2,d2)) * D(n1,d1) * P(n1, d1, λ, theta1) )^((totalLayers - 1)/2) * inv(D(1,45))*D(n1,theta1)
    end
    # Consider: break the return lines into like 3 functions so it isn't so long
end

# determine the reflectivity of the light hitting the mirror
function reflect(n1,n2,d1,d2,dSub,totalLayers,Polarization,λ)
    matr = mirror_stack(n1,n2,d1,d2,dSub,totalLayers,Polarization,λ)
    return ((norm(matr[3]/matr[4])))^2
end

# plot it
function r_plot(n1,n2,d1,d2,dSub,totalLayers,Polarization,λ)
    # reflectivity across spectrum
    y = reflect.(n1, n2, d1, d2, dSub, totalLayers, Polarization, λ)
    
    # plot it
    plot(λ, y, ylims=(-0.1, 1.1), title="Reflectivity of $Polarization Polarized Light", xlabel="Wavelength [nm]", ylabel="Intensity [%]")

    savefig("reflectivity_" * "$Polarization" * ".png")
end
#-------------------------------------------------------------------------------------------
# TEST

# nm wavelength over visible spectrum
λ = 400:0.1:700;

# reflectivity 
r_plot(1.46, 4.6, 52, 52, 1000, 21, "TM", λ);


