<script lang="ts">
  // @ts-nocheck

  import CardMedia from "../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProfile from "../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
</script>

<div class="p-10 flex flex-col gap-10 bg-gray-50 min-h-screen">
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
      videoUrl: "https://cdn.pixabay.com/video/2026/10/19/383541_large.mp4",
    }}
    buttonProps={{
      text: "Get Started",
    }}
  />

  <CardProfile
    logoProps={{
      src: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?q=80&w=880&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
    }}
    titleProps={{
      title: "Chester Maldonado",
    }}
    descriptionProps={{
      description: "Supporting startups and SaaS in enhancing their conversions. I'm always open to thoughtful design conversations.",
    }}
    thumbnailVideoProps={{
      imageUrl: "https://images.unsplash.com/photo-1777896193454-8b4863264f0b?q=80&w=1171&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=NHwxMjA3fDA%3D",
      videoUrl: "https://cdn.pixabay.com/video/2026/10/19/383541_large.mp4",
    }}
    tagsProps={{
      tags: [
        { title: "768", description: "Posts" },
        { title: "24.3k", description: "Likes" },
        { title: "12k", description: "Followers" }
      ]
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
</div>
