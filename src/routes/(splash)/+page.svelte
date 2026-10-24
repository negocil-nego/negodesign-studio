<script lang="ts">
  // @ts-nocheck

  import CardMedia from "../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProfile from "../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
</script>

<CardProfile
  variant="02"
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
