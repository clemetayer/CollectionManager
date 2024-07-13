<script lang="ts">
    import { defineComponent } from "vue";
    import { CollectionService } from '../../service/collection.service';
    import type { Collection } from "@/models/collection.model";

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"remove-child-collection-dialog",
        props:['collection'],
        data() {
            return {
                open: false,
                childCollection: {} as Collection,
            }
        },
        methods: {
            openDialog() {
                this.open = true;
            },
            setChildCollection(childCollection : Collection) {
                this.childCollection = childCollection;
            },
            onConfirm(result : any) {
                if(result) {
                    collectionService.removeChildCollection(this.collection.deezer_id, this.childCollection.deezer_id)
                        .then(_ => {
                            this.$emit("refresh-data");
                        });
                }
            },
        }
    });
</script>

<template>
    <ui-dialog v-model="open" @confirm="onConfirm" data-cy="remove-child-collection-dialog">
        <ui-dialog-title>Remove the child collection ?</ui-dialog-title>
        <ui-dialog-content>
            Do you really want to remove the collection <a v-bind:href="childCollection.url">{{ childCollection.name }}</a> from the collection <a v-bind:href="collection.url">{{ collection.name }}</a>? <br/><br/>
            This will not remove the distant Deezer playlist, and the remaining tracks should be removed from the playlist manually.
        </ui-dialog-content>
        <ui-dialog-actions data-cy="remove-child-collection-dialog-actions"></ui-dialog-actions>
    </ui-dialog>
</template>