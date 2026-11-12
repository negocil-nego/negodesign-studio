<script lang="ts">
  import PageLogin from "../../../../../project/ui/pages/login/simple/ui/PageLogin.svelte";
  import PageLoginDual from "../../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.svelte";
  import PageRegister from "../../../../../project/ui/pages/register/simple/ui/PageRegister.svelte";
  import PageRegisterDual from "../../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.svelte";
  import PageOtp from "../../../../../project/ui/pages/otp/simple/ui/PageOtp.svelte";
  import PageOtpDual from "../../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.svelte";
  import ComponentScreenGroup from "../shared/ComponentScreenGroup.svelte";
  import { templateScreenNavigation } from "../shared/template-screen-navigation";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  import pageLoginSvelte from "../../../../../project/ui/pages/login/simple/ui/PageLogin.svelte?raw";
  import pageLoginReact from "../../../../../project/ui/pages/login/simple/ui/PageLogin.tsx?raw";
  import pageLoginVue from "../../../../../project/ui/pages/login/simple/ui/PageLogin.vue?raw";
  import pageLoginFlutter from "../../../../../project/ui/pages/login/simple/ui/PageLogin.dart?raw";
  import pageLoginDualSvelte from "../../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.svelte?raw";
  import pageLoginDualReact from "../../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.tsx?raw";
  import pageLoginDualVue from "../../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.vue?raw";
  import pageLoginDualFlutter from "../../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.dart?raw";
  import pageRegisterSvelte from "../../../../../project/ui/pages/register/simple/ui/PageRegister.svelte?raw";
  import pageRegisterReact from "../../../../../project/ui/pages/register/simple/ui/PageRegister.tsx?raw";
  import pageRegisterVue from "../../../../../project/ui/pages/register/simple/ui/PageRegister.vue?raw";
  import pageRegisterFlutter from "../../../../../project/ui/pages/register/simple/ui/PageRegister.dart?raw";
  import pageRegisterDualSvelte from "../../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.svelte?raw";
  import pageRegisterDualReact from "../../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.tsx?raw";
  import pageRegisterDualVue from "../../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.vue?raw";
  import pageRegisterDualFlutter from "../../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.dart?raw";
  import pageOtpSvelte from "../../../../../project/ui/pages/otp/simple/ui/PageOtp.svelte?raw";
  import pageOtpReact from "../../../../../project/ui/pages/otp/simple/ui/PageOtp.tsx?raw";
  import pageOtpVue from "../../../../../project/ui/pages/otp/simple/ui/PageOtp.vue?raw";
  import pageOtpFlutter from "../../../../../project/ui/pages/otp/simple/ui/PageOtp.dart?raw";
  import pageOtpDualSvelte from "../../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.svelte?raw";
  import pageOtpDualReact from "../../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.tsx?raw";
  import pageOtpDualVue from "../../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.vue?raw";
  import pageOtpDualFlutter from "../../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.dart?raw";

  const group = templateScreenNavigation.find(
    (item) => item.type === "authentication",
  )!;
</script>

<div class="space-y-5">
  <ComponentScreenGroup
    label={group.label}
    badge={group.badge}
    description={group.description}
    count={group.count}
  />

  <Terminal id="page-login" componentId="page-login" name="PageLogin">
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageLogin />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/login/simple/ui/PageLogin.svelte"
        source={pageLoginSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/login/simple/ui/PageLogin.tsx"
        source={pageLoginReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/login/simple/ui/PageLogin.vue"
        source={pageLoginVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/login/simple/ui/PageLogin.dart"
        source={pageLoginFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-login-dual"
    componentId="page-login-dual"
    name="PageLoginDual"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageLoginDual />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/login/dual-section/ui/PageLoginDual.svelte"
        source={pageLoginDualSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/login/dual-section/ui/PageLoginDual.tsx"
        source={pageLoginDualReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/login/dual-section/ui/PageLoginDual.vue"
        source={pageLoginDualVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/login/dual-section/ui/PageLoginDual.dart"
        source={pageLoginDualFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-register"
    componentId="page-register"
    name="PageRegister"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageRegister />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/register/simple/ui/PageRegister.svelte"
        source={pageRegisterSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/register/simple/ui/PageRegister.tsx"
        source={pageRegisterReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/register/simple/ui/PageRegister.vue"
        source={pageRegisterVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/register/simple/ui/PageRegister.dart"
        source={pageRegisterFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-register-dual"
    componentId="page-register-dual"
    name="PageRegisterDual"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageRegisterDual />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/register/dual-section/ui/PageRegisterDual.svelte"
        source={pageRegisterDualSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/register/dual-section/ui/PageRegisterDual.tsx"
        source={pageRegisterDualReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/register/dual-section/ui/PageRegisterDual.vue"
        source={pageRegisterDualVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/register/dual-section/ui/PageRegisterDual.dart"
        source={pageRegisterDualFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal id="page-otp" componentId="page-otp" name="PageOtp">
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageOtp />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/otp/simple/ui/PageOtp.svelte"
        source={pageOtpSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/otp/simple/ui/PageOtp.tsx"
        source={pageOtpReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/otp/simple/ui/PageOtp.vue"
        source={pageOtpVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/otp/simple/ui/PageOtp.dart"
        source={pageOtpFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-otp-dual"
    componentId="page-otp-dual"
    name="PageOtpDual"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.55] min-h-70 max-h-115 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageOtpDual />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}
    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/otp/dual-section/ui/PageOtpDual.svelte"
        source={pageOtpDualSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/otp/dual-section/ui/PageOtpDual.tsx"
        source={pageOtpDualReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/otp/dual-section/ui/PageOtpDual.vue"
        source={pageOtpDualVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/otp/dual-section/ui/PageOtpDual.dart"
        source={pageOtpDualFlutter}
      />
    {/snippet}
  </Terminal>
</div>
