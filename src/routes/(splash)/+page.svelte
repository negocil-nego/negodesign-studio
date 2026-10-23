<script lang="ts">
  // @ts-nocheck

  import CardMedia from "../../project/ui/molecules/card/media/ui/CardMedia.svelte";
</script>

<CardMedia
  logoProps={{
    src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?q=80&w=880&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
    alt: "logo",
  }}
  titleProps={{
    title: "Negodesign",
  }}
  descriptionProps={{
    description: "Negodesign is a design system for building user interfaces.",
  }}
  ratingProps={{
    rating: 5,
  }}
  cartButtonProps={{
    onClick: () => alert("Add to Cart"),
  }}
  favoriteButtonProps={{
    onClick: () => alert("Favorite"),
  }}
  thumbnailVideoProps={{
    imageUrl:
      "https://images.unsplash.com/photo-1777896193454-8b4863264f0b?q=80&w=1171&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
    videoUrl: "https://www.youtube.com/watch?v=-_mBODVnyhU",
  }}
  buttonProps={{
    text: "Get Started",
  }}
/>
