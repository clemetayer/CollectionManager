<script lang="ts">
    import { defineComponent } from 'vue';
    import type {Collection} from '../models/collection.model';
    import { CollectionService } from '../service/collection.service';
    import { type SelectOption } from "../models/balmui.model"

    let collectionService = new CollectionService();
    

    export default defineComponent({
        name:"collection-data",
        data() {
            return {
                selectedCollection: "",
                collection: {} as Collection,
                openAddChildCollectionDialog: false,
                childCollectionURL : '',
                fromUrlChecked: false,
                collections: [] as SelectOption[],
                childCollectionSelection: ''
            }
        },
        methods: {
            displayCollection(collectionId : string) : void {
                console.log("displaying collections from id : " + collectionId);
                collectionService.getCollection(collectionId).then(collectionRes => {
                    this.collection = collectionRes;
                });
            },
            updateCollection() : void {
                collectionService.updateCollection(this.collection.deezer_id);
            },
            addChildCollection() : void {
                this.openAddChildCollectionDialog = true;
            },
            onAddChildCollectionConfirm(result : any) : void {
                if(result) {
                    if(this.fromUrlChecked){
                        let child_id_split = this.childCollectionURL.split("/");
                        let child_id = child_id_split[child_id_split.length - 1];
                        collectionService.addChildCollection(this.collection.deezer_id, child_id);
                        console.log("adding child collection " + child_id + " to " + this.collection.deezer_id + " from URL"); 
                    }
                    else {
                        collectionService.addChildCollection(this.collection.deezer_id, this.childCollectionSelection);
                        console.log("adding child collection " + this.childCollectionSelection + " to " + this.collection.deezer_id + " from URL");
                    }
                }
            },
            onSelectChildCollection(pSelectedOption : any) {
                this.childCollectionSelection = pSelectedOption.value;
            },
        }
    });
</script>

<template>
    <div>
        <h2>
            <v-container>
                <a v-bind:href="collection.url">{{ collection.name }}</a>
                <ui-icon-button icon="add" @click="addChildCollection()"></ui-icon-button>
            </v-container>
        </h2>
        <ui-button @click="updateCollection">Update collection</ui-button>
        <ui-grid>
            <ui-grid-cell></ui-grid-cell>
            <ui-grid-cell>
                <ui-list>
                    <ui-item v-for="childCol in collection.children_col" :key="childCol">
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="childCol.url">{{ childCol.name }}</a>
                            </ui-item-text1>
                        </ui-item-text-content>
                        <ui-item-last-content>
                            <ui-icon>queue_music</ui-icon>
                        </ui-item-last-content>
                    </ui-item>
                </ui-list>
                <ui-list>
                    <ui-item v-for="track in collection.tracks" :key="track">
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="track.link">{{ track.title }}</a>
                            </ui-item-text1>
                            <ui-item-text2>
                                {{ track.artist }}
                            </ui-item-text2>
                        </ui-item-text-content>
                        <ui-item-last-content>
                            <ui-icon>music_note</ui-icon>
                        </ui-item-last-content>
                    </ui-item>
                </ui-list>
            </ui-grid-cell>
            <ui-grid-cell></ui-grid-cell>
        </ui-grid>
        <ui-dialog v-model="openAddChildCollectionDialog" @confirm="onAddChildCollectionConfirm">
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
                        <ui-select fullwidth v-model="childCollectionSelection" :options="collections" @selected="onSelectChildCollection($event)"></ui-select>
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
    </div>
</template>