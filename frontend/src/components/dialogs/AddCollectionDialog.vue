<script lang="ts">
    import { defineComponent } from "vue";
    import { CollectionService } from '../../service/collection.service';

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"add-collection-dialog",
        data() {
            return {
                open: false,
                collectionName: "",
                fromPlaylistChecked: false,
                fromPlaylist: null
            }
        },
        methods: {
            openDialog() {
                this.open = true;
            },
            onConfirm(result : any) {
                if(result){
                    if(this.fromPlaylistChecked) {
                        this.collectionName = "";
                    }
                    else {
                        this.fromPlaylist = null;
                    }
                    console.log("add collection " + this.collectionName + " from playlist " + this.fromPlaylist);
                    collectionService.initCollection(this.collectionName, this.fromPlaylist).then(_ => {
                        this.$emit("refresh-data");
                    });
                }
            },
        }
    });
</script>

<template>
    <ui-dialog v-model="open" @confirm="onConfirm">
            <ui-dialog-title>Enter the collection name</ui-dialog-title>
            <ui-dialog-content>
                <form method="dialog">
                    <div v-if="fromPlaylistChecked">
                        <label>From playlist :</label>
                        <ui-textfield
                            v-model="fromPlaylist"
                            helper-text-id="from-playlist-field-helper-text"
                        >
                            Deezer playlist URL
                        </ui-textfield>
                    </div>
                    <div v-else>
                        <label>Collection name :</label>
                        <ui-textfield
                            v-model="collectionName"
                            :disabled="fromPlaylistChecked"
                            helper-text-id="collection-name-field-helper-text"
                        >
                            Collection name
                        </ui-textfield>
                    </div>
                    <div>
                        <ui-form-field>
                            <label>From playlist :</label>
                            <ui-checkbox v-model="fromPlaylistChecked" input-id="checkbox"></ui-checkbox>
                        </ui-form-field>
                    </div>
                </form>
            </ui-dialog-content>
            <ui-dialog-actions></ui-dialog-actions>
        </ui-dialog>
</template>