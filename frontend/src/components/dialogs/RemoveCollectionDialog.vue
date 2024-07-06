<script lang="ts">
    import { defineComponent } from "vue";
    import { CollectionService } from '../../service/collection.service';

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"remove-collection-dialog",
        props:['collection'],
        data() {
            return {
                open: false,
            }
        },
        methods: {
            openDialog() {
                this.open = true;
            },
            onConfirm(result : any) {
                if(result) {
                    collectionService.removeCollection(this.collection.deezer_id);
                }
            },
        }
    });
</script>

<template>
    <ui-dialog v-model="open" @confirm="onConfirm">
        <ui-dialog-title>Remove the collection ?</ui-dialog-title>
        <ui-dialog-content>
            Do you really want to remove the collection <a v-bind:href="collection.url">{{ collection.name }}</a> from the database ? <br/><br/>
            This will not remove the distant Deezer playlist.
        </ui-dialog-content>
        <ui-dialog-actions></ui-dialog-actions>
    </ui-dialog>
</template>