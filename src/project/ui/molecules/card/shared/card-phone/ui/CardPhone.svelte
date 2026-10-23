<script lang="ts">
    import type { CardPhoneProps } from "../data/types";

    let { phone }: CardPhoneProps = $props();
</script>
<div class="card-phone" id="card-phone">
    <div class="card-phone-number">{phone}</div>
</div>
