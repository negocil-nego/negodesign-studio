<script lang="ts">
  // @ts-nocheck

  import CardMedia from "../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProfile from "../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
  import CardProduct from "../../project/ui/molecules/card/product/ui/CardProduct.svelte";
</script>

<CardMedia
  variant="01"
  logoProps={{
    src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?q=80&w=880&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
  }}
  titleProps={{
    title: "Chester Maldonado",
  }}
  descriptionProps={{
    description:
      "Supporting startups and SaaS in enhancing their conversions. I'm always open to thoughtful design conversations.",
  }}
  
  thumbnailVideoProps={{
    videoProps:{
      src: "https://cdn.pixabay.com/video/2026/10/19/383541_large.mp4",
      controls: false,
    },
    imageProps: {
      src:  "https://images.unsplash.com/photo-1777896193454-8b4863264f0b?q=80&w=1171&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
    }
  }}
  tagsProps={{
    tags: [
      { title: "768", description: "Posts" },
      { title: "24.3k", description: "Likes" },
      { title: "12k", description: "Followers" },
    ],
  }}
  buttonProps={{
    text: "Get in touch",
    onClick: () => alert("Get in touch"),
  }}
  cartButtonProps={{
    onClick: () => alert("Cart"),
  }}
  favoriteButtonProps={{
    onClick: () => alert("Favorite"),
  }}
/>

<CardProduct
  variant="01"
  logoProps={{
    src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?q=80&w=880&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
  }}
  titleProps={{
    title: "Aurora Wireless Headphones",
  }}
  ratingProps={{
    rating: 4,
  }}
  priceProps={{
    number: 129,
  }}
  descriptionProps={{
    description:
      "Imersão sonora com cancelamento ativo de ruído, 40 horas de autonomia e um design leve para o uso diário.",
  }}
  imageProps={{
    src: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?q=80&w=1170&auto=format&fit=crop",
  }}
  minFileProps={[
    {
      imageProps: {
        src: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?q=80&w=400&auto=format&fit=crop",
      },
      active: true,
    },
    {
      imageProps: {
        src: "https://images.unsplash.com/photo-1484704849700-f032a568e944?q=80&w=400&auto=format&fit=crop",
      },
    },
    {
      imageProps: {
        src: "https://images.unsplash.com/photo-1546435770-a3e426bf472b?q=80&w=400&auto=format&fit=crop",
      },
    },
  ]}
  cartButtonProps={{
    onClick: () => alert("Cart"),
  }}
  favoriteButtonProps={{
    onClick: () => alert("Favorite"),
  }}
  buttonProps={{
    text: "Add to cart",
    onClick: () => alert("Add to cart"),
  }}
/>
