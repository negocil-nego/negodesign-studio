Na funcionalidade de conexão ao banco de dados em src/routes/(editor)/main/database/+page.svelte
deve ter uma opção para gerar o backend para uma aplicação frontend,
irá funcionar da seguinte forma será pedido o caminho da aplicação
frontend (sveltekit, nuxt, nextjs) de backend sendo (spring boot, nestjs, laravel, asp.net core) a ideia é ler 
podes se basear nesta organização de projecto (
    /home/sedrac-slc/Documentos/project/negoturismo/api/negoturismo-api-admin/src/main/java/com/negocil/negoturismo/admin/feature
), para construir a o backend baseado no padrão de feature semelhante ao nestjs 
ou spring modulith.

Para aplicação spring deves perguntar de irás utilizar o como repositorio (orm) o Spring DATA JPA ou Spring JDBC usando API nova, para rotas de get deves retornar com paginação.

Para Nestjs deves escolher se as consultas dos repositórios serão gerado por
prisma, drizzle ou typeorm.

Para Laravel deves escolher se as consultas dos repositórios serão gerado por
Eloquent, Query Builder ou Raw SQL.

Para ASP.net core deves escolher se as consultas dos repositórios serão gerado por
Entity Framework Core, Dapper ou Raw SQL.

Irás fazer os crud das tabelas fazend o método insert, update, delete, findAll,
seguindo o padrão se criar as controller -> service -> repositório, ao gerar o backend.

Para integrar com o frontend deves para sveltekit, nuxtjs, nextjs deves perguntar
se irá seguir o padrão de usar BFF ou chamar o backend diretamente, e para esses frameworks deve utilizar os que eles trazem de nativo para requisição, no caso do nuxt tem o useFetch, e o sveltekit, nextjs utiliza mesmo o fetch api, mais também deves perguntar e utilizador que utilizar o axios ou fetch api,, e caso é para usar o TanStack Query de cada framework (tanstack/react-query, tanstack/svelte-query, tanstack/vue-query)

e para construir as telas de crud no frontend deves usar o componente "src/project/ui/pages/admin/ui", que irás utilizar como sidebar os menus corresponder
as tabelas que tens na tua base dados (sidebarProps), o datatableProps (será construído com base ao rota do backend que retorna os registro páginados) e o
  let { datatableProps, sidebarProps, modalFormProps, menuProps }: AdminProps =
 (irás criar este formulário para o processo de criação e edição)
 --------------------------------------------------------------------------------

 Mas também a parte do banco de dados caso seja escolhido o sveltekit, nuxtjs, nextjs, podes utilizar a mesma aplicação  como backend criando as rotas de api fazendo conexão direta com a base de dados (postgres, mysql, sqlite), deveras escolher opção de gerar tudo numa apliacão ou devider em duas aplicações.


 --------------------------------------------------------------------------------

Essas opção de gerar o backend para uma aplicação frontend posso escolher todas tabelas ou apenas algumas.

---------------------------------------------------------------------------------
Neste projecto já tens a funções que copia os components para outros projecto em rust e para esse processo dessas funcionalidade deves colocar no rust a criar os reajust nos projectos frontend e backend e organiza bem "src-tauri/src"
de modo que é lógica de spring boot, sveltekit, e outras tecnologias fica  separado qual é a estrategia a utilizar para os repositórios escolha do orm também fica separado, nada de god class, god code.

>-----------------------------------------------

para a página "src/routes/(editor)/main/openapi/+page.svelte"
quando receber as o openai podes gerar também aplicação frontend
Para integrar com o frontend deves para sveltekit, nuxtjs, nextjs deves perguntar se irá seguir o padrão de usar BFF ou chamar o backend diretamente, e para esses frameworks deve utilizar os que eles trazem de nativo para requisição, no caso do nuxt tem o useFetch, e o sveltekit, nextjs utiliza mesmo o fetch api, mais também deves perguntar e utilizador que utilizar o axios ou fetch api,, e caso é para usar o TanStack Query de cada framework (tanstack/react-query, tanstack/svelte-query, tanstack/vue-query), irás criar os dtos e response baseado no openapi, posso escollher qual endpoint será feito a integração<
para gerar o AdminCrud, 
para iddentificar na página "src/routes/(editor)/main/openapi/+page.svelte" seja os endpoint que para criar o AdminCrud no projecto fronted senja se são API Resteful [GET; POST; PUT/PATCH, DELETE] ou seguem o padrão de nomenclatura de métodos como 
[
  GET (/list, /all, os nomes de enpoint mais comun),
  POST (/create, /add,os nomes de enpoint mais comun),
  PUT/PATCH (/update, /edit, /save,os nomes de enpoint mais comun),
  DELETE (/delete, /remove,os nomes de enpoint mais comun)
]
