import mpmath

# Set the precision to 220 digits (extra for guard digits to prevent rounding errors)
mpmath.mp.dps = 220

# Method 1: Calculate sin(pi/4)
pi_val = mpmath.pi
angle = pi_val / 4
sin_value = mpmath.sin(angle)

# Method 2: Calculate 1 / sqrt(2)
sqrt2_val = mpmath.sqrt(2)
reciprocal_sqrt2 = 1 / sqrt2_val

# Compare (they should be identical)
print("sin(pi/4) to 200 digits:")
print(mpmath.nstr(sin_value, 200))
print("\n1/sqrt(2) to 200 digits:")
print(mpmath.nstr(reciprocal_sqrt2, 200))
print("\nAre they identical?", sin_value == reciprocal_sqrt2) 
