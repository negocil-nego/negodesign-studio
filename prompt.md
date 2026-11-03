Irás criar os componentes do carrossel

src\project\ui\organisms\section\carousel\card\media
CarouselCardMedia
src\project\ui\organisms\section\carousel\card\product
CarouselCardProduct
src\project\ui\organisms\section\carousel\card\profile
CarouselCardProfile

Esses compos vão receber SectionCarouselButtons com a diferença que o "children" será uma lista
de CardProfileProps -> CarouselCardProfile, CardProductProps -> CarouselCardProduct, CardMediaProps -> CarouselCardMedia

e do mesmo jeito que tens o SectionCarouselButtons irás criar SectionGrid (que irá apresentar em grid onde grid-cols-1, md:grid-cols-3, 2xl:grid-cols-4, 3xl:grid-cols-5)

src\project\ui\organisms\section\grid\card\media
GridCardMedia
src\project\ui\organisms\section\grid\card\product
GridCardProduct
src\project\ui\organisms\section\grid\card\profile
GridCardProfile

e do mesmo jeito que tens o SectionCarouselButtons irás criar SectionGridCarousel (
ele é uma mistura de apresentar os componente em carousel em grupo de grid onde irei indicar, a linha e coluna
)
src\project\ui\organisms\section\grid-carousel\card\media
GridCarouselCardMedia
src\project\ui\organisms\section\grid-carousel\card\product
GridCarouselCardProduct
src\project\ui\organisms\section\grid-carousel\card\profile
GridCarouselCardProfile
