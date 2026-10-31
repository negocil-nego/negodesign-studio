import SectionCarouselButtons from "../../../../../../project/ui/organisms/section/carousel/buttons/ui/SectionCarouselButtons";

export default function SectionCarouselButtonsUsage() {
  const testimonials = [
    { name: "Blake Star", role: "COO, blackstar.com", image: "/img/programmer-01.jpg", rating: 5 },
    { name: "David Gilmore", role: "CTO, darkside.ai", image: "/img/ux-01.jpg", rating: 5 },
    { name: "Gerard White", role: "COO, exo4emo.ai", image: "/img/programmer-02.jpg", rating: 5 },
    { name: "Azuny", role: "COO, studio.example", image: "/img/ux-01.jpg", rating: 4 },
  ];

  return (
    <SectionCarouselButtons
      titleProps={{ text: "O que dizem os nossos clientes" }}
      descriptionProps={{ text: "Histórias de pessoas que usam a plataforma no dia-a-dia." }}
    >
      {testimonials.map((person) => (
        <article key={person.name} className="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
          <img src={person.image} alt={person.name} className="h-56 w-full object-cover" />
          <div className="space-y-2 p-4">
            <p className="text-amber-400" aria-label={`${person.rating} estrelas`}>{"★".repeat(person.rating)}{"☆".repeat(5 - person.rating)}</p>
            <h3 className="font-semibold text-zinc-100">{person.name}</h3>
            <p className="text-xs text-zinc-400">{person.role}</p>
          </div>
        </article>
      ))}
    </SectionCarouselButtons>
  );
}
