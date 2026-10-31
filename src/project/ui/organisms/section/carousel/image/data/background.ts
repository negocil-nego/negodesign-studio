const cssGradientPattern = /^(linear|radial|conic)-gradient\(/i;
const gradientClassPattern = /\b(bg-gradient|bg-linear|bg-radial|bg-conic|from-|via-|to-)/;
const cssColorPattern = /^(#|rgb\(|rgba\(|hsl\(|hsla\(|transparent$|[a-z]+$)/i;

export function getCarouselBackground(backgroundColor = "") {
  const value = backgroundColor.trim();
  const isCssGradient = cssGradientPattern.test(value);
  const isGradientClass = gradientClassPattern.test(value);
  const isCssColor = cssColorPattern.test(value);

  return {
    isCssGradient,
    isGradientClass,
    isCssColor,
    backgroundClass:
      isCssColor || isCssGradient || isGradientClass ? "" : backgroundColor,
    backgroundGradientClass: isGradientClass ? backgroundColor : "",
  };
}
