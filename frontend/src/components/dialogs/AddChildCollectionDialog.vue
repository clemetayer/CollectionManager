<script lang="ts">
    import { defineComponent } from "vue";
    import { CollectionService } from '../../service/collection.service';

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"add-collection-dialog",
        props:['collections', 'collection'],
        data() {
            return {
                open: false,
                fromUrlChecked: false,
                childCollection: '',
                childCollectionURL : '',
            }
        },
        methods: {
            openDialog() {
                this.open = true;
            },
            onConfirm(result : any) {
                if(result) {
                    if(this.fromUrlChecked){
                        let child_id_split = this.childCollectionURL.split("/");
                        let child_id = child_id_split[child_id_split.length - 1];
                        collectionService.addChildCollection(this.collection.deezer_id, child_id).then(_ => {
                            this.$emit("refresh-data");
                        });
                        console.log("adding child collection " + child_id + " to " + this.collection.deezer_id + " from URL"); 
                    }
                    else {
                        collectionService.addChildCollection(this.collection.deezer_id, this.childCollection).then(_ => {
                            this.$emit("refresh-data");
                        });
                        console.log("adding child collection " + this.childCollection + " to " + this.collection.deezer_id + " from URL");
                    }
                }
            },
            onSelectChildCollection(pSelectedOption : any) {
                this.childCollection = pSelectedOption.value;
            },
        }
    });

</script>

<template>
    <ui-dialog v-model="open" @confirm="onConfirm">
        <ui-dialog-title>Enter the child deezer playlist url or select from the dropdown</ui-dialog-title>
        <ui-dialog-content>
            <form method="dialog">
                <div>
                    <ui-form-field>
                        <label>From url :</label>
                        <ui-checkbox v-model="fromUrlChecked" input-id="checkbox"></ui-checkbox>
                    </ui-form-field>
                </div>
                <div v-if="!fromUrlChecked">
                    <ui-select fullwidth v-model="childCollection" :options="collections" @selected="onSelectChildCollection($event)"></ui-select>
                </div>
                <div v-if="fromUrlChecked">
                    <ui-textfield
                        v-model="childCollectionURL"
                        helper-text-id="from-playlist-field-helper-text"
                    >
                        Deezer playlist URL
                    </ui-textfield>
                </div>
            </form>
        </ui-dialog-content>
        <ui-dialog-actions></ui-dialog-actions>
    </ui-dialog>
</template>